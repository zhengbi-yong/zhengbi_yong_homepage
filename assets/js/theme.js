/**
 * 主题切换功能
 * 管理暗色模式和亮色模式的切换
 */

/**
 * 初始化主题（从 localStorage 读取或使用系统偏好）
 */
function initTheme() {
    const savedTheme = localStorage.getItem('theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    const theme = savedTheme || (systemPrefersDark ? 'dark' : 'light');
    applyTheme(theme);
    
    return theme;
}

/**
 * 应用主题
 * @param {string} theme - 'light' 或 'dark'
 */
function applyTheme(theme) {
    const html = document.documentElement;
    if (theme === 'dark') {
        html.classList.add('dark');
    } else {
        html.classList.remove('dark');
    }
    localStorage.setItem('theme', theme);
}

/**
 * 切换主题
 */
function toggleTheme() {
    const html = document.documentElement;
    const isDark = html.classList.contains('dark');
    const newTheme = isDark ? 'light' : 'dark';
    applyTheme(newTheme);
    return newTheme;
}

/**
 * 获取当前主题
 */
function getCurrentTheme() {
    const html = document.documentElement;
    return html.classList.contains('dark') ? 'dark' : 'light';
}

// 页面加载时初始化主题
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initTheme);
} else {
    initTheme();
}

// 监听系统主题变化
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (!localStorage.getItem('theme')) {
        applyTheme(e.matches ? 'dark' : 'light');
    }
});

// 导出函数供其他脚本使用
if (typeof window !== 'undefined') {
    window.initTheme = initTheme;
    window.applyTheme = applyTheme;
    window.toggleTheme = toggleTheme;
    window.getCurrentTheme = getCurrentTheme;
}

