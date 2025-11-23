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

    if (!element) {
        return;
    }

    // 查找所有代码块
    const codeBlocks = element.querySelectorAll('pre code');
    codeBlocks.forEach((block) => {
        // 清除之前的高亮标记
        if (block.dataset.highlighted) {
            delete block.dataset.highlighted;
        }
        // 移除旧的类名和内容
        block.classList.remove('hljs');
        // 重新高亮
        try {
            hljs.highlightElement(block);
        } catch (error) {
            // 如果高亮失败，尝试使用 highlightAll
            console.warn('单个元素高亮失败，使用全局高亮:', error);
        }
    });
}

/**
 * 高亮所有代码块（全局）
 */
function highlightAll() {
    if (typeof hljs === 'undefined') {
        console.warn('highlight.js 未加载');
        return;
    }
    
    // 清除所有已高亮的标记
    const highlightedBlocks = document.querySelectorAll('code[data-highlighted]');
    highlightedBlocks.forEach((block) => {
        delete block.dataset.highlighted;
    });
    
    // 重新高亮所有代码块
    hljs.highlightAll();
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

// 使用 MutationObserver 监听 DOM 变化，自动高亮新添加的代码块
if (typeof MutationObserver !== 'undefined') {
    const observer = new MutationObserver((mutations) => {
        let shouldHighlight = false;
        
        mutations.forEach((mutation) => {
            if (mutation.addedNodes.length > 0) {
                // 检查是否有新的代码块被添加
                mutation.addedNodes.forEach((node) => {
                    if (node.nodeType === 1) { // Element node
                        if (node.tagName === 'CODE' || node.tagName === 'PRE') {
                            shouldHighlight = true;
                        } else if (node.querySelectorAll) {
                            const codeBlocks = node.querySelectorAll('pre code, code');
                            if (codeBlocks.length > 0) {
                                shouldHighlight = true;
                            }
                        }
                    }
                });
            }
        });
        
        if (shouldHighlight) {
            // 延迟高亮，确保 DOM 完全更新
            setTimeout(() => {
                if (typeof hljs !== 'undefined') {
                    highlightAll();
                }
            }, 150);
        }
    });
    
    // 开始观察
    if (document.body) {
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    } else {
        // 如果 body 还不存在，等待它创建
        const bodyObserver = new MutationObserver(() => {
            if (document.body) {
                observer.observe(document.body, {
                    childList: true,
                    subtree: true
                });
                bodyObserver.disconnect();
            }
        });
        bodyObserver.observe(document.documentElement, {
            childList: true
        });
    }
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
                        setTimeout(() => {
                            highlightAll();
                        }, 100);
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
    window.highlightAll = highlightAll;
}
