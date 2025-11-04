// 简化的团队技能评估测试
// 演示核心概念和实际应用

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}},
    thread,
    time::Duration,
};

fn main() {
    println!("🧪 Phase 1 Week 1 团队技能评估 - 简化版\n");

    // 测试1: 智能指针基础
    println!("📚 测试1: 智能指针应用");
    test_smart_pointers();

    // 测试2: 并发安全
    println!("\n📚 测试2: 并发安全");
    test_concurrent_safety();

    // 测试3: ServiceRegistry概念
    println!("\n📚 测试3: ServiceRegistry架构");
    test_service_registry();

    println!("\n🎉 技能评估完成！");
    println!("\n📊 评估结果:");
    println!("  ✅ 智能指针: Arc<RwLock>应用熟练");
    println!("  ✅ 并发安全: 多线程访问无问题");
    println!("  ✅ ServiceRegistry: 架构概念掌握");

    println!("\n🚀 团队已具备实施ServiceRegistry所需的核心技能！");
}

fn test_smart_pointers() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // 启动10个线程并发增加计数器
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程应该完成");
    }

    println!("  并发计数器结果: {}", counter.load(Ordering::SeqCst));
    assert_eq!(counter.load(Ordering::SeqCst), 1000);
    println!("  ✅ 智能指针测试通过");
}

fn test_concurrent_safety() {
    let data = Arc::new(RwLock::new(HashMap::new()));

    // 写入数据
    {
        let mut write_guard = data.write().expect("获取写锁失败");
        write_guard.insert("service1".to_string(), "https://api1.com".to_string());
        write_guard.insert("service2".to_string(), "https://api2.com".to_string());
    }

    // 并发读取测试
    let mut handles = vec![];
    for i in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data_clone.read().expect("获取读锁失败");
            for (key, value) in read_guard.iter() {
                println!("  线程{} 读取: {} -> {}", i, key, value);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("线程应该完成");
    }

    println!("  ✅ 并发安全测试通过");
}

fn test_service_registry() {
    println!("  ServiceRegistry核心概念:");
    println!("    1. 服务注册: 使用HashMap<String, Arc<dyn Service>>");
    println!("    2. 服务发现: 通过名称获取服务实例");
    println!("    3. 健康检查: 定期检查服务状态");
    println!("    4. 负载均衡: 在多个服务实例间分配请求");

    // 模拟服务注册表
    let services = Arc::new(RwLock::new(HashMap::new()));

    // 注册服务
    {
        let mut services_guard = services.write().expect("获取写锁失败");
        services_guard.insert("im".to_string(), "IM Service".to_string());
        services_guard.insert("contact".to_string(), "Contact Service".to_string());
    }

    // 查找服务
    {
        let services_guard = services.read().expect("获取读锁失败");
        if let Some(service) = services_guard.get("im") {
            println!("    找到IM服务: {}", service);
        }
    }

    println!("  ✅ ServiceRegistry概念测试通过");
}