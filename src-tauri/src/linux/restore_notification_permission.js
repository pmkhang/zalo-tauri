(() => {
  if (location.origin !== "https://chat.zalo.me" || !("Notification" in window)) {
    return;
  }

  let notificationWasEnabled = false;
  try {
    notificationWasEnabled = Object.keys(localStorage).some(
      (key) => key.endsWith("_askNoti") && localStorage.getItem(key) === "1",
    );
  } catch {
    return;
  }

  if (!notificationWasEnabled) {
    return;
  }

  void Notification.requestPermission();

  // Zalo checks this synchronously before WebKit finishes restoring the native
  // origin permission. Reflect the persisted user choice during that window.
  try {
    Object.defineProperty(Notification, "permission", {
      configurable: true,
      get: () => "granted",
    });
  } catch {
    // WebContext permission remains the native fallback.
  }
})();
