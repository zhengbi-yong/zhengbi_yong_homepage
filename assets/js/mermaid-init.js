/**
 * Mermaid 图表渲染初始化
 * 使用 ES 模块导入 Mermaid
 */

let mermaidInstance = null;
let mermaidInitialized = false;
let mermaidInitPromise = null;

/**
 * 初始化 Mermaid
 */
async function initMermaid() {
    // 如果正在初始化，返回 Promise
    if (mermaidInitPromise) {
        return mermaidInitPromise;
    }
    
    // 如果已经初始化，直接返回
    if (mermaidInitialized && mermaidInstance) {
        return mermaidInstance;
    }

    mermaidInitPromise = (async () => {
        try {
            // 动态导入 Mermaid（ES 模块）
            const mermaidModule = await import('https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs');
            mermaidInstance = mermaidModule.default;
            
            // 初始化 Mermaid
            mermaidInstance.initialize({
                startOnLoad: false, // 手动控制渲染
                theme: 'default',
                securityLevel: 'loose',
                fontFamily: 'inherit',
                flowchart: {
                    useMaxWidth: true,
                    htmlLabels: true,
                    curve: 'basis'
                }
            });

            mermaidInitialized = true;
            console.log('Mermaid 初始化成功');
            
            // 立即渲染现有的图表
            await renderAllMermaid();
            
            return mermaidInstance;
        } catch (error) {
            console.error('Mermaid 初始化失败:', error);
            mermaidInitPromise = null;
            return null;
        }
    })();
    
    return mermaidInitPromise;
}

/**
 * 渲染所有 Mermaid 图表
 */
async function renderAllMermaid() {
    // 确保 Mermaid 已初始化
    if (!mermaidInstance) {
        await initMermaid();
    }
    
    if (!mermaidInstance) {
        console.warn('Mermaid 实例不可用');
        return;
    }

    try {
        // 查找所有未渲染的 Mermaid 图表
        const mermaidElements = document.querySelectorAll('.mermaid:not([data-processed])');
        
        if (mermaidElements.length === 0) {
            return;
        }

        console.log(`找到 ${mermaidElements.length} 个 Mermaid 图表需要渲染`);

        // 渲染所有图表
        for (const element of mermaidElements) {
            try {
                // 标记为已处理
                element.setAttribute('data-processed', 'true');
                
                // 获取图表代码
                const graphDefinition = element.textContent.trim();
                
                if (!graphDefinition) {
                    console.warn('Mermaid 图表代码为空');
                    continue;
                }

                // 确保元素有 ID（Mermaid 需要）
                const id = element.id || `mermaid-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
                if (!element.id) {
                    element.id = id;
                }
                
                // 保存原始文本内容
                const originalText = element.textContent.trim();
                
                // 渲染图表
                try {
                    await mermaidInstance.run({
                        nodes: [element],
                        suppressErrors: true
                    });
                    
                    // 等待一小段时间，确保 SVG 已创建
                    await new Promise(resolve => setTimeout(resolve, 100));
                    
                    // 检查是否成功渲染（查找 SVG）
                    const svg = element.querySelector('svg');
                    if (svg) {
                        console.log('Mermaid 图表渲染成功，SVG 已创建');
                        // 确保 SVG 可见
                        svg.style.display = 'block';
                        svg.style.maxWidth = '100%';
                        svg.style.height = 'auto';
                    } else {
                        console.warn('Mermaid 图表渲染后未找到 SVG 元素');
                        console.warn('元素内容:', element.innerHTML);
                        console.warn('原始文本:', originalText);
                    }
                } catch (renderError) {
                    console.error('Mermaid run() 调用失败:', renderError);
                    throw renderError;
                }
            } catch (error) {
                console.error('渲染单个 Mermaid 图表失败:', error);
                // 移除标记，允许重试
                element.removeAttribute('data-processed');
            }
        }
    } catch (error) {
        console.error('渲染 Mermaid 图表失败:', error);
    }
}

/**
 * 手动渲染 Mermaid 图表
 * @param {Element} element - 包含 Mermaid 代码的 DOM 元素
 */
async function renderMermaid(element) {
    if (!mermaidInstance) {
        await initMermaid();
    }
    
    if (!mermaidInstance || !element) {
        return;
    }

    try {
        // 如果元素还没有被处理，则渲染
        if (!element.hasAttribute('data-processed')) {
            element.setAttribute('data-processed', 'true');
            await mermaidInstance.run({
                nodes: [element],
                suppressErrors: true
            });
        }
    } catch (error) {
        console.error('Mermaid 渲染失败:', error);
        element.removeAttribute('data-processed');
    }
}

// 等待 DOM 加载完成
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        initMermaid();
    });
} else {
    initMermaid();
}

// 使用 MutationObserver 监听 DOM 变化，自动渲染新的 Mermaid 图表
if (typeof MutationObserver !== 'undefined') {
    const observer = new MutationObserver((mutations) => {
        let shouldRender = false;
        
        mutations.forEach((mutation) => {
            if (mutation.addedNodes.length > 0) {
                // 检查是否有新的 Mermaid 元素被添加
                mutation.addedNodes.forEach((node) => {
                    if (node.nodeType === 1) { // Element node
                        if (node.classList && node.classList.contains('mermaid')) {
                            shouldRender = true;
                        } else if (node.querySelectorAll) {
                            const mermaidElements = node.querySelectorAll('.mermaid:not([data-processed])');
                            if (mermaidElements.length > 0) {
                                shouldRender = true;
                            }
                        }
                    }
                });
            }
        });
        
        if (shouldRender) {
            // 延迟渲染，确保 DOM 完全更新
            setTimeout(() => {
                renderAllMermaid().catch((error) => {
                    console.error('自动渲染 Mermaid 图表失败:', error);
                });
            }, 200);
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

// 导出函数供其他脚本使用
if (typeof window !== 'undefined') {
    window.initMermaid = initMermaid;
    window.renderMermaid = renderMermaid;
    window.renderAllMermaid = renderAllMermaid;
}
