use open_lark::prelude::*;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration from environment
    let app_id = env::var("LARK_APP_ID").expect("LARK_APP_ID must be set");
    let app_secret = env::var("LARK_APP_SECRET").expect("LARK_APP_SECRET must be set");

    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    info!("🚀 Starting Lark Event Handler Example");
    info!("This example demonstrates how to handle various Lark/Feishu events");

    // Create event dispatcher with comprehensive event handling
    let _dispatcher = create_event_dispatcher()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create dispatcher: {}", e))?;

    info!("✅ Event dispatcher created successfully");
    info!("📝 Event handlers registered for:");
    info!("   • IM Events: Message received, recalled, chat created/updated/disbanded, member added/removed");
    info!("   • Contact Events: User created/updated/deleted, department created/updated/deleted");
    info!("   • Drive Events: File created/updated/deleted");
    info!("   • Calendar Events: Event created");
    info!("   • VC Events: Meeting started/ended, participant joined/left");
    info!("   • Approval Events: Instance created/approved/rejected");

    // In a real application, you would:
    // 1. Start a WebSocket connection to receive events
    // 2. Use dispatcher.dispatch(event_payload) to handle incoming events
    // 3. The registered handlers will be called automatically

    info!("🎯 Event dispatcher is ready to handle incoming events!");
    info!("💡 To use this in your application:");
    info!("   1. Establish a WebSocket connection to Lark");
    info!("   2. When you receive an event, call dispatcher.dispatch(payload)");
    info!("   3. Your registered event handlers will be executed automatically");

    // Keep the program running to demonstrate
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    info!("📋 Example completed successfully");

    Ok(())
}

/// Creates and configures an EventDispatcher with handlers for all major event types
async fn create_event_dispatcher() -> Result<EventDispatcherHandler, String> {
    let mut dispatcher = EventDispatcherHandler::builder();

    // =====================
    // IM (Instant Message) Events
    // =====================
    info!("🔧 Registering IM event handlers...");

    // Message received
    dispatcher = dispatcher.register_p2_im_message_receive_v1(|event| {
        info!(
            "📨 Message received: {} from {}",
            event.event.message.content, event.event.sender.sender_id.user_id
        );
    })?;

    // Message recalled
    dispatcher = dispatcher.register_p2_im_message_recalled_v1(|event| {
        warn!(
            "↩️ Message recalled: {} by {}",
            event.event.message_id,
            event
                .event
                .operator
                .operator_id
                .user_id
                .unwrap_or_else(|| "".to_string())
        );
    })?;

    // Chat created
    dispatcher = dispatcher.register_p2_im_chat_created_v1(|event| {
        info!(
            "💬 New chat created: {} by {}",
            event.event.name.unwrap_or("Unnamed chat".to_string()),
            event
                .event
                .creator
                .user_id
                .user_id
                .unwrap_or_else(|| "".to_string())
        );
    })?;

    // Chat updated
    dispatcher = dispatcher.register_p2_im_chat_updated_v1(|event| {
        info!("📝 Chat updated: {}", event.event.chat_id);
    })?;

    // Chat disbanded
    dispatcher = dispatcher.register_p2_im_chat_disbanded_v1(|event| {
        warn!(
            "❌ Chat disbanded: {} by {}",
            event.event.chat_id,
            event
                .event
                .operator
                .operator_id
                .user_id
                .unwrap_or_else(|| "".to_string())
        );
    })?;

    // Member added to chat
    dispatcher = dispatcher.register_p2_im_chat_member_user_added_v1(|event| {
        info!(
            "➕ Member added to chat {}: {} users",
            event.event.chat_id,
            event.event.users.len()
        );
    })?;

    // Member removed from chat
    dispatcher = dispatcher.register_p2_im_chat_member_user_deleted_v1(|event| {
        info!(
            "➖ Member removed from chat {}: {} users",
            event.event.chat_id,
            event.event.users.len()
        );
    })?;

    // =====================
    // Contact Events
    // =====================
    info!("🔧 Registering Contact event handlers...");

    // User created
    dispatcher = dispatcher.register_p2_contact_user_created_v3(|event| {
        info!(
            "👤 New user created: {} ({})",
            event
                .event
                .object
                .user
                .name
                .unwrap_or_else(|| "Unknown".to_string()),
            event.event.object.user.user_id
        );
    })?;

    // User updated
    dispatcher = dispatcher.register_p2_contact_user_updated_v3(|event| {
        info!("🔄 User updated: {}", event.event.object.user.user_id);
    })?;

    // User deleted
    dispatcher = dispatcher.register_p2_contact_user_deleted_v3(|event| {
        warn!("🗑️ User deleted: {}", event.event.object.user.user_id);
    })?;

    // Department created
    dispatcher = dispatcher.register_p2_contact_department_created_v3(|event| {
        info!(
            "🏢 Department created: {} ({})",
            event.event.object.department.name, event.event.object.department.department_id
        );
    })?;

    // Department updated
    dispatcher = dispatcher.register_p2_contact_department_updated_v3(|event| {
        info!(
            "🔄 Department updated: {}",
            event.event.object.department.department_id
        );
    })?;

    // Department deleted
    dispatcher = dispatcher.register_p2_contact_department_deleted_v3(|event| {
        warn!(
            "🗑️ Department deleted: {}",
            event.event.object.department.department_id
        );
    })?;

    // =====================
    // Drive (Cloud Docs) Events
    // =====================
    info!("🔧 Registering Drive event handlers...");

    // File created
    dispatcher = dispatcher.register_p2_drive_file_created_v1(|event| {
        info!(
            "📄 File created: {} ({})",
            event.event.object.file.name, event.event.object.file.file_token
        );
    })?;

    // File updated
    dispatcher = dispatcher.register_p2_drive_file_updated_v1(|event| {
        info!("📝 File updated: {}", event.event.object.file.file_token);
    })?;

    // File deleted
    dispatcher = dispatcher.register_p2_drive_file_deleted_v1(|event| {
        warn!(
            "🗑️ File deleted: {} ({})",
            event.event.object.file.name, event.event.object.file.file_token
        );
    })?;

    // =====================
    // Calendar Events
    // =====================
    info!("🔧 Registering Calendar event handlers...");

    // Calendar event created
    dispatcher = dispatcher.register_p2_calendar_event_created_v4(|event| {
        info!(
            "📅 Calendar event created: {} at {}",
            event.event.object.calendar_event.summary,
            event
                .event
                .object
                .calendar_event
                .start_time
                .timestamp
                .unwrap_or_else(|| "".to_string())
        );
    })?;

    // =====================
    // VC (Video Conference) Events
    // =====================
    info!("🔧 Registering VC event handlers...");

    // Meeting started
    dispatcher = dispatcher.register_p2_vc_meeting_started_v1(|event| {
        info!(
            "🎥 Meeting started: {} hosted by {}",
            event.event.object.meeting.topic,
            event
                .event
                .object
                .meeting
                .host
                .display_name
                .unwrap_or_else(|| "Unknown".to_string())
        );
    })?;

    // Meeting ended
    dispatcher = dispatcher.register_p2_vc_meeting_ended_v1(|event| {
        info!(
            "🏁 Meeting ended: {} (Duration: {} seconds)",
            event.event.object.meeting.topic,
            event.event.object.meeting.duration.unwrap_or(0)
        );
    })?;

    // Participant joined
    dispatcher = dispatcher.register_p2_vc_meeting_participant_joined_v1(|event| {
        info!(
            "👋 Participant joined meeting {}: {}",
            event.event.object.meeting.meeting_id,
            event
                .event
                .object
                .participant
                .display_name
                .unwrap_or_else(|| "Unknown".to_string())
        );
    })?;

    // Participant left
    dispatcher = dispatcher.register_p2_vc_meeting_participant_left_v1(|event| {
        info!(
            "👋 Participant left meeting {}: {} (Duration: {} seconds)",
            event.event.object.meeting.meeting_id,
            event
                .event
                .object
                .participant
                .display_name
                .unwrap_or_else(|| "Unknown".to_string()),
            event.event.object.participant.total_duration.unwrap_or(0)
        );
    })?;

    // =====================
    // Approval Events
    // =====================
    info!("🔧 Registering Approval event handlers...");

    // Approval instance created
    dispatcher = dispatcher.register_p2_approval_instance_created_v4(|event| {
        info!(
            "📋 Approval instance created: {} by {}",
            event
                .event
                .object
                .instance
                .approval_name
                .unwrap_or_else(|| "Unknown".to_string()),
            event.event.object.instance.user_id
        );
    })?;

    // Approval instance approved
    dispatcher = dispatcher.register_p2_approval_instance_approved_v4(|event| {
        info!(
            "✅ Approval instance approved: {} ({})",
            event
                .event
                .object
                .instance
                .approval_name
                .unwrap_or_else(|| "Unknown".to_string()),
            event.event.object.instance.instance_id
        );
    })?;

    // Approval instance rejected
    dispatcher = dispatcher.register_p2_approval_instance_rejected_v4(|event| {
        warn!(
            "❌ Approval instance rejected: {} ({})",
            event
                .event
                .object
                .instance
                .approval_name
                .unwrap_or_else(|| "Unknown".to_string()),
            event.event.object.instance.instance_id
        );
    })?;

    info!("✅ All event handlers registered successfully!");

    // Build the final dispatcher
    Ok(dispatcher.build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_dispatcher_creation() {
        // This test ensures the event dispatcher can be created without errors
        let result = create_event_dispatcher().await;
        assert!(result.is_ok(), "Event dispatcher creation should succeed");
    }

    #[test]
    fn test_event_handler_completeness() {
        // This test documents all the events we've implemented
        let implemented_events = vec![
            // IM Events
            "p2_im_message_receive_v1",
            "p2_im_message_recalled_v1",
            "p2_im_chat_created_v1",
            "p2_im_chat_updated_v1",
            "p2_im_chat_disbanded_v1",
            "p2_im_chat_member_user_added_v1",
            "p2_im_chat_member_user_deleted_v1",
            // Contact Events
            "p2_contact_user_created_v3",
            "p2_contact_user_updated_v3",
            "p2_contact_user_deleted_v3",
            "p2_contact_department_created_v3",
            "p2_contact_department_updated_v3",
            "p2_contact_department_deleted_v3",
            // Drive Events
            "p2_drive_file_created_v1",
            "p2_drive_file_updated_v1",
            "p2_drive_file_deleted_v1",
            // Calendar Events
            "p2_calendar_event_created_v4",
            // VC Events
            "p2_vc_meeting_started_v1",
            "p2_vc_meeting_ended_v1",
            "p2_vc_meeting_participant_joined_v1",
            "p2_vc_meeting_participant_left_v1",
            // Approval Events
            "p2_approval_instance_created_v4",
            "p2_approval_instance_approved_v4",
            "p2_approval_instance_rejected_v4",
        ];

        assert_eq!(
            implemented_events.len(),
            20,
            "We should have 20 implemented events"
        );
        println!(
            "Successfully implemented {} events",
            implemented_events.len()
        );
    }
}
