/**
 * 图片懒加载功能
 * 使用 Intersection Observer API 实现图片懒加载
 */

/**
 * 初始化图片懒加载
 */
function initLazyLoad() {
    // 检查是否支持 Intersection Observer
    if (!('IntersectionObserver' in window)) {
        // 不支持 Intersection Observer，直接加载所有图片
        const images = document.querySelectorAll('img[loading="lazy"]');
        images.forEach((img) => {
            if (img.dataset.src) {
                img.src = img.dataset.src;
            }
        });
        return;
    }

    // 创建 Intersection Observer
    const imageObserver = new IntersectionObserver((entries, observer) => {
        entries.forEach((entry) => {
            if (entry.isIntersecting) {
                const img = entry.target;
                
                // 如果有 data-src，使用它作为真实 src
                if (img.dataset.src) {
                    img.src = img.dataset.src;
                    img.removeAttribute('data-src');
                }
                
                // 添加加载完成的类
                img.classList.add('loaded');
                
                // 停止观察
                observer.unobserve(img);
            }
        });
    }, {
        rootMargin: '50px' // 提前 50px 开始加载
    });

    // 观察所有懒加载图片
    const lazyImages = document.querySelectorAll('img[loading="lazy"]');
    lazyImages.forEach((img) => {
        imageObserver.observe(img);
    });
}

// 等待 DOM 加载完成
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initLazyLoad);
} else {
    initLazyLoad();
}

// 导出函数供其他脚本使用
if (typeof window !== 'undefined') {
    window.initLazyLoad = initLazyLoad;
}

