(function () {
  'use strict';

  var root = document.documentElement;
  var themeButton = document.getElementById('theme-toggle');

  function syncThemeButton() {
    if (!themeButton) return;
    var dark = root.classList.contains('theme-dark');
    themeButton.setAttribute('aria-pressed', dark ? 'true' : 'false');
    themeButton.setAttribute('aria-label', dark ? 'Switch to light theme' : 'Switch to dark theme');
  }

  if (themeButton) {
    syncThemeButton();
    themeButton.addEventListener('click', function () {
      var dark = root.classList.toggle('theme-dark');
      try { localStorage.setItem('tf-theme', dark ? 'dark' : 'light'); } catch (e) {}
      syncThemeButton();
    });
  }

  /* Mobile navigation */
  var sidebar = document.getElementById('sidebar');
  var backdrop = document.getElementById('sidebar-backdrop');
  var openButton = document.querySelector('[data-sidebar-toggle]');
  var closeButton = document.querySelector('[data-sidebar-close]');

  function setSidebar(open) {
    if (!sidebar) return;
    sidebar.classList.toggle('is-open', open);
    if (backdrop) backdrop.hidden = !open;
    if (openButton) openButton.setAttribute('aria-expanded', open ? 'true' : 'false');
    document.body.classList.toggle('sidebar-open', open);
  }

  if (openButton) openButton.addEventListener('click', function () { setSidebar(true); });
  if (closeButton) closeButton.addEventListener('click', function () { setSidebar(false); });
  if (backdrop) backdrop.addEventListener('click', function () { setSidebar(false); });

  if (sidebar) {
    sidebar.addEventListener('click', function (event) {
      if (event.target.closest('a')) setSidebar(false);
    });
  }

  document.addEventListener('keydown', function (event) {
    if (event.key === 'Escape') {
      setSidebar(false);
      var results = document.getElementById('search-results');
      if (results) results.innerHTML = '';
    }
  });

  window.addEventListener('resize', function () {
    if (window.innerWidth > 980) setSidebar(false);
  });

  /* Favorites are keyed by stable team IDs, never by display names. */
  var FAVORITES_KEY = 'tf-favorites';

  function readFavorites() {
    try {
      var parsed = JSON.parse(localStorage.getItem(FAVORITES_KEY) || '[]');
      return Array.isArray(parsed) ? parsed.filter(function (id) { return typeof id === 'string'; }) : [];
    } catch (e) {
      return [];
    }
  }

  function writeFavorites(ids) {
    try { localStorage.setItem(FAVORITES_KEY, JSON.stringify(ids)); } catch (e) {}
  }

  function paintFavorites() {
    var favorites = readFavorites();
    var lookup = Object.create(null);
    favorites.forEach(function (id) { lookup[id] = true; });
    document.querySelectorAll('[data-favorite-id]').forEach(function (button) {
      var id = button.getAttribute('data-favorite-id');
      var active = Boolean(lookup[id]);
      button.classList.toggle('is-favorite', active);
      button.setAttribute('aria-pressed', active ? 'true' : 'false');
      button.textContent = active ? '★' : '☆';
      var name = button.getAttribute('data-team-name') || 'team';
      button.setAttribute('aria-label', (active ? 'Remove favorite: ' : 'Add favorite: ') + name);
    });
  }

  document.addEventListener('click', function (event) {
    var button = event.target.closest('[data-favorite-id]');
    if (!button) return;
    event.preventDefault();
    var id = button.getAttribute('data-favorite-id');
    if (!id) return;
    var favorites = readFavorites();
    var index = favorites.indexOf(id);
    if (index >= 0) favorites.splice(index, 1);
    else favorites.push(id);
    writeFavorites(favorites);
    paintFavorites();
  });

  paintFavorites();

  /* Search dropdown visibility; HTMX fills the results container. */
  var searchInput = document.getElementById('global-search');
  var searchResults = document.getElementById('search-results');
  var blurTimer = null;

  function showSearch() {
    if (!searchResults) return;
    if (searchResults.innerHTML.trim() !== '') searchResults.classList.add('is-open');
  }

  if (searchInput && searchResults) {
    searchInput.addEventListener('focus', function () {
      clearTimeout(blurTimer);
      showSearch();
    });
    searchInput.addEventListener('input', function () {
      if (searchInput.value.trim().length < 2) {
        searchResults.innerHTML = '';
        searchResults.classList.remove('is-open');
      }
    });
    searchInput.addEventListener('blur', function () {
      blurTimer = setTimeout(function () {
        searchResults.classList.remove('is-open');
        searchResults.innerHTML = '';
      }, 220);
    });
    searchResults.addEventListener('mouseenter', function () { clearTimeout(blurTimer); });
    searchResults.addEventListener('mouseleave', function () {
      blurTimer = setTimeout(function () { searchResults.classList.remove('is-open'); }, 180);
    });
    document.body.addEventListener('htmx:afterSwap', function (event) {
      if (event.target === searchResults) {
        searchResults.classList.toggle('is-open', searchResults.innerHTML.trim() !== '');
      }
    });
  }
})();
