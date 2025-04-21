(function() {
    console.log('Attempting to inject script directly...');
    var attemptCount = 0;
    var maxAttempts = 5;
    var interval = 500; // ms

    function applyStylesAndDrag() {
        attemptCount++;
        console.log('Attempt #' + attemptCount);
        try {
            var targetDocument = document;
            var body = targetDocument.body;
            var html = targetDocument.documentElement;

            
            if (body && html) {
                console.log('Found body and html elements.');
                body.style.background = 'unset';
                body.style.backgroundColor = 'transparent';
                body.style.borderRadius = '16px';
                html.style.backgroundColor = 'transparent';
                console.log('Applied background styles.');

                var navBars = targetDocument.querySelectorAll('nav[class*="horizontal-nav-bar"]');
                var backgroundContainers = targetDocument.querySelectorAll('div[class*="background-container"]');

                if (navBars.length > 0) {
                    navBars.forEach(navBar => {
                        navBar.setAttribute('data-tauri-drag-region', '');
                    });
                    console.log('Draggable region set on ' + navBars.length + ' nav bar(s).');
                } else {
                    console.warn('Nav bar elements not found yet for drag region.');
                    return false;
                }

                if (backgroundContainers.length > 0) {
                     backgroundContainers.forEach(container => {
                        container.style.backgroundColor = 'transparent';
                        container.style.background = 'unset';
                     });
                    console.log('Background styles applied to ' + backgroundContainers.length + ' container(s).');
                } else {
                    console.warn('Background container elements not found yet.');
                    return false;
                }

                return true;
            } else {
                console.warn('Body or html element not found yet.');
                return false;
            }
        } catch (e) {
            console.error('Error during injection attempt:', e);
            return true;
        }
    }

    function scheduleAttempt() {
        if (!applyStylesAndDrag() && attemptCount < maxAttempts) {
            console.log('Retrying in ' + interval + 'ms...');
            setTimeout(scheduleAttempt, interval);
        } else if (attemptCount >= maxAttempts) {
             console.error('Max attempts reached. Injection failed.');
        }
    }

    scheduleAttempt();

    const observer = new MutationObserver((mutationsList, observer) => {
        console.log('DOM changed, re-running applyStylesAndDrag...');
        applyStylesAndDrag();
    });

    function startObserver() {
        if (document.body) {
             observer.observe(document.body, { childList: true, subtree: true });
             console.log('MutationObserver started on document.body');
        } else {
            console.log('Body not ready for observer, retrying...');
            setTimeout(startObserver, 100);
        }
    }
    startObserver();

})(); 