/**
 * Mermaid 图表渲染初始化
 * 使用 ES 模块导入 Mermaid
 */

let mermaidInitialized = false;

/**
 * 初始化 Mermaid
 */
async function initMermaid() {
    // 如果已经初始化，直接返回
    if (mermaidInitialized) {
        return;
    }

    try {
        // 动态导入 Mermaid（ES 模块）
        const mermaid = await import('https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs');
        
        // 初始化 Mermaid
        mermaid.default.initialize({
            startOnLoad: true,
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
    } catch (error) {
        console.error('Mermaid 初始化失败:', error);
    }
}

/**
 * 手动渲染 Mermaid 图表
 * @param {Element} element - 包含 Mermaid 代码的 DOM 元素
 */
async function renderMermaid(element) {
    if (!mermaidInitialized) {
        await initMermaid();
    }

    try {
        const mermaid = await import('https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs');
        await mermaid.default.run({
            nodes: [element],
            suppressErrors: true
        });
    } catch (error) {
        console.error('Mermaid 渲染失败:', error);
    }
}

// 等待 DOM 加载完成
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initMermaid);
} else {
    initMermaid();
}

// 导出函数供其他脚本使用
if (typeof window !== 'undefined') {
    window.initMermaid = initMermaid;
    window.renderMermaid = renderMermaid;
}

