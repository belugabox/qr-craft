// Theme switcher for Beer CSS
// Initialisation immédiate pour éviter le flash (FOUC)
(function() {
    function applyTheme(theme) {
        const html = document.documentElement;
        
        // Appliquer sur html immédiatement
        html.classList.remove('light', 'dark');
        
        if (theme === 'auto' || !theme) {
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            const systemTheme = prefersDark ? 'dark' : 'light';
            html.classList.add(systemTheme);
        } else {
            html.classList.add(theme);
        }
        
        // Appliquer aussi sur body quand il sera disponible
        if (document.body) {
            document.body.classList.remove('light', 'dark');
            if (theme === 'auto' || !theme) {
                const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                const systemTheme = prefersDark ? 'dark' : 'light';
                document.body.classList.add(systemTheme);
            } else {
                document.body.classList.add(theme);
            }
        }
    }
    
    // Appliquer le thème sauvegardé ou auto au chargement
    const savedTheme = localStorage.getItem('app-theme') || 'auto';
    applyTheme(savedTheme);
    
    // Réappliquer sur body quand le DOM est prêt
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            applyTheme(savedTheme);
        });
    }
})();

window.setTheme = function(theme) {
    const html = document.documentElement;
    const body = document.body;
    
    // Sauvegarder le choix
    localStorage.setItem('app-theme', theme);
    
    // Remove existing theme classes
    html.classList.remove('light', 'dark');
    if (body) body.classList.remove('light', 'dark');
    
    // Apply new theme
    if (theme === 'auto') {
        // Use system preference
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        const systemTheme = prefersDark ? 'dark' : 'light';
        html.classList.add(systemTheme);
        if (body) body.classList.add(systemTheme);
        console.log('Theme set to: auto (system prefers:', systemTheme + ')');
    } else {
        html.classList.add(theme);
        if (body) body.classList.add(theme);
        console.log('Theme set to:', theme);
    }
};
