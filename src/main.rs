use rquickjs::{AsyncContext, AsyncRuntime, context::intrinsic};

#[tokio::main]
async fn main() {
    seg_fault().await;

    no_seg_fault_statement().await;
    no_seg_fault_full_context().await;
}

async fn seg_fault() {
    let runtime = AsyncRuntime::new().unwrap();
    let context = AsyncContext::builder()
        .with::<intrinsic::Eval>()
        .build_async(&runtime)
        .await
        .unwrap();

    context
        .with(|ctx| {
            let code = "let x = 42;".to_string();
            ctx.eval::<rquickjs::Value, _>(code.as_bytes()).unwrap();
        })
        .await;
}

async fn no_seg_fault_statement() {
    let runtime = AsyncRuntime::new().unwrap();
    let context = AsyncContext::builder()
        .with::<intrinsic::Eval>()
        .build_async(&runtime)
        .await
        .unwrap();

    context
        .with(|ctx| {
            let code = "1 + 1;".to_string();
            ctx.eval::<rquickjs::Value, _>(code.as_bytes()).unwrap();
        })
        .await;
}

async fn no_seg_fault_full_context() {
    let runtime = AsyncRuntime::new().unwrap();
    let context = AsyncContext::full(&runtime).await.unwrap();

    context
        .with(|ctx| {
            let code = "let x = 42;".to_string();
            ctx.eval::<rquickjs::Value, _>(code.as_bytes()).unwrap();
        })
        .await;
}
