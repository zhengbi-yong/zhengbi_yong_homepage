/**
 * KaTeX 数学公式渲染初始化
 * 在页面加载后自动渲染数学公式
 */

// 等待 DOM 加载完成
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initKaTeX);
} else {
    initKaTeX();
}

/**
 * 初始化 KaTeX 渲染
 */
function initKaTeX() {
    // 检查 KaTeX 是否已加载
    if (typeof renderMathInElement === 'undefined') {
        console.warn('KaTeX auto-render 未加载，数学公式将不会自动渲染');
        return;
    }

    // 使用全局的 renderMathInElement 函数（来自 KaTeX auto-render）
    renderMathInElement(document.body, {
        delimiters: [
            {left: "$$", right: "$$", display: true},      // 块级公式：$$...$$
            {left: "$", right: "$", display: false},      // 行内公式：$...$
            {left: "\\[", right: "\\]", display: true},   // LaTeX 块级：\[...\]
            {left: "\\(", right: "\\)", display: false}   // LaTeX 行内：\(...\)
        ],
        throwOnError: false,  // 不抛出错误，避免影响页面渲染
        errorColor: '#cc0000',
        strict: false
    });
}

/**
 * 手动触发 KaTeX 渲染（用于动态内容）
 * @param {Element} element - 要渲染的 DOM 元素
 */
function renderMathInElementCustom(element) {
    // 检查 KaTeX 是否已加载
    if (typeof renderMathInElement === 'undefined') {
        console.warn('KaTeX auto-render 未加载');
        return;
    }

    // 使用全局的 renderMathInElement 函数（来自 KaTeX auto-render）
    renderMathInElement(element, {
        delimiters: [
            {left: "$$", right: "$$", display: true},
            {left: "$", right: "$", display: false},
            {left: "\\[", right: "\\]", display: true},
            {left: "\\(", right: "\\)", display: false}
        ],
        throwOnError: false,
        errorColor: '#cc0000',
        strict: false
    });
}

// 导出函数供其他脚本使用（使用不同的名称避免冲突）
if (typeof window !== 'undefined') {
    window.renderMath = renderMathInElementCustom;
    window.initKaTeX = initKaTeX;
}
