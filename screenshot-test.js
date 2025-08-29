const puppeteer = require('puppeteer');
const path = require('path');

async function takeScreenshots() {
    console.log('Starting screenshot tests...');
    const browser = await puppeteer.launch();
    
    try {
        // Desktop viewport
        const desktopPage = await browser.newPage();
        await desktopPage.setViewport({ width: 1280, height: 800 });
        const fileUrl = 'file://' + path.resolve('index.html');
        await desktopPage.goto(fileUrl);
        await desktopPage.waitForTimeout(1000); // Wait for animations
        await desktopPage.screenshot({ path: 'screenshot-desktop.png', fullPage: true });
        console.log('✅ Desktop screenshot saved');
        
        // Mobile viewport (iPhone 12 Pro)
        const mobilePage = await browser.newPage();
        await mobilePage.setViewport({ width: 390, height: 844, isMobile: true });
        await mobilePage.goto(fileUrl);
        await mobilePage.waitForTimeout(1000);
        await mobilePage.screenshot({ path: 'screenshot-mobile.png', fullPage: true });
        console.log('✅ Mobile screenshot saved');
        
        // Check copyright footer
        const footerBounds = await mobilePage.evaluate(() => {
            const footer = document.querySelector('.copyright-footer');
            if (footer) {
                const rect = footer.getBoundingClientRect();
                return {
                    visible: true,
                    height: rect.height,
                    width: rect.width,
                    bottom: rect.bottom,
                    text: footer.textContent.trim()
                };
            }
            return { visible: false };
        });
        
        console.log('\nCopyright footer check:');
        console.log('- Visible:', footerBounds.visible);
        console.log('- Height:', footerBounds.height, 'px');
        console.log('- Width:', footerBounds.width, 'px');
        console.log('- Text:', footerBounds.text);
        
        if (footerBounds.height > 40) {
            console.log('⚠️  Footer might be too tall for mobile');
        } else {
            console.log('✅ Footer height is reasonable');
        }
        
    } finally {
        await browser.close();
    }
    
    console.log('\nScreenshots saved! Check screenshot-desktop.png and screenshot-mobile.png');
}

// Run if this file is executed directly
if (require.main === module) {
    takeScreenshots().catch(console.error);
}

module.exports = { takeScreenshots };