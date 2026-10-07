/**
 * Universal Timetable Backend Adapter
 * 
 * 3-tier progressive resolution strategy:
 *   Tier 1: Native Tauri IPC invoke (Desktop / Mobile apps)
 *   Tier 2: WebAssembly core (Modern web browser execution)
 *   Tier 3: Pure JavaScript engine (Zero-dependency resilient fallback)
 */
(function(window) {
  'use strict';

  var wasmModule = null;
  var wasmLoaded = false;
  var wasmLoadPromise = null;

  var isTauri = typeof window.__TAURI__ !== 'undefined' || typeof window.__TAURI_INTERNALS__ !== 'undefined';

  function tauriInvoke(cmd, args) {
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
      return window.__TAURI__.core.invoke(cmd, args);
    }
    if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
      return window.__TAURI_INTERNALS__.invoke(cmd, args);
    }
    return Promise.reject(new Error('Not in Tauri'));
  }

  function loadWasm() {
    if (wasmLoaded) return Promise.resolve(wasmModule);
    if (wasmLoadPromise) return wasmLoadPromise;
    if (typeof WebAssembly === 'undefined') return Promise.reject(new Error('WASM unsupported'));

    wasmLoadPromise = import('./pkg/timetable_wasm.js')
      .then(function(mod) {
        if (typeof mod.default === 'function') {
          return mod.default().then(function() { return mod; });
        }
        return mod;
      })
      .then(function(mod) {
        wasmModule = mod;
        wasmLoaded = true;
        return mod;
      })
      .catch(function(err) {
        // Fallback gracefully without breaking UI
        return null;
      });

    return wasmLoadPromise;
  }

  if (!isTauri && typeof WebAssembly !== 'undefined') {
    loadWasm().catch(function() {});
  }

  var TimetableBackend = {
    isTauri: function() {
      return isTauri;
    },

    isWasmReady: function() {
      return wasmLoaded && wasmModule !== null;
    },

    getNowNext: async function(fallbackFn) {
      // Tier 1: Tauri IPC
      if (isTauri) {
        try {
          var tauriResult = await tauriInvoke('get_now_next');
          if (tauriResult) return tauriResult;
        } catch (e) {}
      }

      // Tier 2: WebAssembly
      if (wasmLoaded && wasmModule && typeof wasmModule.get_now_next === 'function') {
        try {
          var wasmResult = wasmModule.get_now_next(Date.now());
          if (wasmResult) return wasmResult;
        } catch (e) {}
      }

      // Tier 3: Client JS fallback
      if (typeof fallbackFn === 'function') {
        return fallbackFn();
      }
      if (typeof window.nowInfo === 'function') {
        return window.nowInfo();
      }
      return null;
    },

    getSessionsForDay: async function(dayIndex, fallbackFn) {
      if (isTauri) {
        try {
          var res = await tauriInvoke('get_sessions_for_day', { dayIndex: dayIndex });
          if (res) return res;
        } catch (e) {}
      }

      if (wasmLoaded && wasmModule && typeof wasmModule.get_sessions_for_day === 'function') {
        try {
          var wasmRes = wasmModule.get_sessions_for_day(dayIndex);
          if (wasmRes) return wasmRes;
        } catch (e) {}
      }

      if (typeof fallbackFn === 'function') {
        return fallbackFn(dayIndex);
      }
      if (window.E) {
        return window.E.filter(function(x) { return x[0] === dayIndex; });
      }
      return [];
    },

    getFreeGaps: async function(dayIndex, fallbackFn) {
      if (isTauri) {
        try {
          var res = await tauriInvoke('get_free_gaps', { dayIndex: dayIndex });
          if (res) return res;
        } catch (e) {}
      }

      if (wasmLoaded && wasmModule && typeof wasmModule.get_free_gaps === 'function') {
        try {
          var wasmRes = wasmModule.get_free_gaps(dayIndex);
          if (wasmRes) return wasmRes;
        } catch (e) {}
      }

      if (typeof fallbackFn === 'function') {
        return fallbackFn(dayIndex);
      }
      return [];
    },

    getWeekStats: async function(fallbackFn) {
      if (isTauri) {
        try {
          var res = await tauriInvoke('get_week_stats');
          if (res) return res;
        } catch (e) {}
      }

      if (wasmLoaded && wasmModule && typeof wasmModule.get_week_stats === 'function') {
        try {
          var wasmRes = wasmModule.get_week_stats();
          if (wasmRes) return wasmRes;
        } catch (e) {}
      }

      if (typeof fallbackFn === 'function') {
        return fallbackFn();
      }
      return null;
    },

    exportCalendarICS: async function(alarmMinutes, fallbackFn) {
      if (isTauri) {
        try {
          var res = await tauriInvoke('export_calendar_ics', { alarmMinutes: alarmMinutes });
          if (res) return res;
        } catch (e) {}
      }

      if (wasmLoaded && wasmModule && typeof wasmModule.export_calendar_ics === 'function') {
        try {
          var wasmRes = wasmModule.export_calendar_ics(alarmMinutes);
          if (wasmRes) return wasmRes;
        } catch (e) {}
      }

      if (typeof fallbackFn === 'function') {
        return fallbackFn(alarmMinutes);
      }
      return null;
    }
  };

  window.TimetableBackend = TimetableBackend;
})(window);
