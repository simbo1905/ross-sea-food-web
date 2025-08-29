use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::viewport::Viewport;
use chromiumoxide::page::ScreenshotParams;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 Layout Screenshot Test");
    println!("========================");

    // Launch browser
    let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    
    tokio::spawn(async move {
        while handler.next().await.is_some() {}
    });

    // Load the page
    let page_path = std::env::current_dir()?.join("index.html");
    let url = format!("file://{}", page_path.display());
    
    // Test desktop viewport
    println!("\n📱 Testing Desktop Layout (1280x800)...");
    let desktop_page = browser.new_page(url.clone()).await?;
    desktop_page.set_viewport(Viewport {
        width: 1280,
        height: 800,
        device_scale_factor: Some(1.0),
        emulating_mobile: false,
        ..Default::default()
    }).await?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Take desktop screenshot
    let desktop_screenshot = desktop_page.screenshot(ScreenshotParams::default()).await?;
    std::fs::write("screenshot-desktop-layout.png", desktop_screenshot)?;
    println!("✅ Desktop screenshot saved to screenshot-desktop-layout.png");
    
    // Check footer on desktop
    let desktop_footer = desktop_page.evaluate(r#"
        (() => {
            const footer = document.querySelector('.copyright-footer');
            if (footer) {
                const rect = footer.getBoundingClientRect();
                return {
                    visible: true,
                    height: rect.height,
                    bottom: window.innerHeight - rect.bottom,
                    text: footer.textContent.trim()
                };
            }
            return { visible: false };
        })()
    "#).await?.into_value::<serde_json::Value>()?;
    
    println!("Desktop footer: {:?}", desktop_footer);
    
    // Test mobile viewport (iPhone 14)
    println!("\n📱 Testing Mobile Layout (390x844)...");
    let mobile_page = browser.new_page(url).await?;
    mobile_page.set_viewport(Viewport {
        width: 390,
        height: 844,
        device_scale_factor: Some(3.0),
        emulating_mobile: true,
        is_mobile: Some(true),
        has_touch: Some(true),
        ..Default::default()
    }).await?;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Take mobile screenshot
    let mobile_screenshot = mobile_page.screenshot(ScreenshotParams::default()).await?;
    std::fs::write("screenshot-mobile-layout.png", mobile_screenshot)?;
    println!("✅ Mobile screenshot saved to screenshot-mobile-layout.png");
    
    // Check footer on mobile
    let mobile_footer = mobile_page.evaluate(r#"
        (() => {
            const footer = document.querySelector('.copyright-footer');
            if (footer) {
                const rect = footer.getBoundingClientRect();
                const styles = window.getComputedStyle(footer);
                return {
                    visible: true,
                    height: rect.height,
                    fontSize: styles.fontSize,
                    padding: styles.padding,
                    bottom: window.innerHeight - rect.bottom,
                    overlapping: rect.bottom > window.innerHeight,
                    text: footer.textContent.trim().substring(0, 100)
                };
            }
            return { visible: false };
        })()
    "#).await?.into_value::<serde_json::Value>()?;
    
    println!("Mobile footer: {:?}", mobile_footer);
    
    // Check if footer is reasonable size
    if let Some(height) = mobile_footer.get("height").and_then(|h| h.as_f64()) {
        if height > 50.0 {
            println!("⚠️  Footer height ({:.1}px) might be too tall for mobile", height);
        } else {
            println!("✅ Footer height ({:.1}px) is reasonable", height);
        }
    }
    
    browser.close().await?;
    
    println!("\n✅ Layout test complete! Check the screenshot files.");
    
    Ok(())
}