const handleError = ({ description, fatal, message, type }) =>
gtag("event", "exception", { description, fatal, message, type });



/** Google Tag Manager */
// (function (w, d, s, l, i) {
//   w[l] = w[l] || [];
//   w[l].push({ "gtm.start": new Date().getTime(), event: "gtm.js" });
//   var f = d.getElementsByTagName(s)[0],
//     j = d.createElement(s),
//     dl = l != "dataLayer" ? "&l=" + l : "";
//   j.async = true;
//   j.src = "https://www.googletagmanager.com/gtm.js?id=" + i + dl;
//   f.parentNode.insertBefore(j, f);
// })(window, document, "script", "dataLayer", "GTM-KDG8N4P");
// /** End Google Tag Manager */
// window.dataLayer.push(function () {
//   this.reset();
// });

// window.dataLayer.push(function () {
//   const existingTime = this.get("time");
//   if (existingTime !== null) {
//     // Change behavior based on whether or not this value exists...
//   } else {
//     // ...
//   }
// });

// window.dataLayer.push(function () {
//   this.set("time", new Date());
// });
