/**
 * 代码高亮初始化
 * 使用 highlight.js 高亮代码块
 */

/**
 * 初始化代码高亮
 */
function initHighlight() {
    // 检查 highlight.js 是否已加载
    if (typeof hljs === 'undefined') {
        console.warn('highlight.js 未加载，代码高亮将不会工作');
        return;
    }

    // 高亮所有代码块
    hljs.highlightAll();
}

/**
 * 手动高亮指定元素中的代码块
 * @param {Element} element - 要处理的 DOM 元素
 */
function highlightElement(element) {
    if (typeof hljs === 'undefined') {
        console.warn('highlight.js 未加载');
        return;
    }

    // 查找所有代码块
    const codeBlocks = element.querySelectorAll('pre code');
    codeBlocks.forEach((block) => {
        hljs.highlightElement(block);
    });
}

// 等待 DOM 和 highlight.js 加载完成
function waitForHighlightJS() {
    if (typeof hljs !== 'undefined') {
        initHighlight();
    } else {
        // 如果 highlight.js 还未加载，等待一段时间后重试
        setTimeout(waitForHighlightJS, 100);
    }
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', waitForHighlightJS);
} else {
    waitForHighlightJS();
}

// 监听主题变化，重新高亮（如果需要切换主题）
function rehighlightOnThemeChange() {
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
                const target = mutation.target;
                if (target === document.documentElement) {
                    // 主题变化，重新高亮代码块
                    if (typeof hljs !== 'undefined') {
                        hljs.highlightAll();
                    }
                }
            }
        });
    });

    observer.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['class']
    });
}

// 启动主题变化监听
rehighlightOnThemeChange();

// 导出函数供其他脚本使用
if (typeof window !== 'undefined') {
    window.initHighlight = initHighlight;
    window.highlightElement = highlightElement;
}

