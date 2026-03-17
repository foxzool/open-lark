pub mod whiteboard;

// whiteboard 模块显式导出

pub use whiteboard::{
    CreatePlantumlNodeBodyV1, CreatePlantumlNodeRequestV1, CreatePlantumlNodeResponseV1,
    CreateWhiteboardNodeBodyV1, CreateWhiteboardNodeRequestV1, CreateWhiteboardNodeResponseV1,
    DownloadWhiteboardAsImageBodyV1, DownloadWhiteboardAsImageRequestV1,
    DownloadWhiteboardAsImageResponseV1, GetWhiteboardThemeRequestV1, GetWhiteboardThemeResponseV1,
    ListWhiteboardNodeRequestV1, ListWhiteboardNodeResponseV1, NodePosition,
    UpdateWhiteboardThemeBodyV1, UpdateWhiteboardThemeRequestV1, UpdateWhiteboardThemeResponseV1,
    WhiteboardNode, WhiteboardTheme,
};
