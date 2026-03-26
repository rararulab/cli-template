/**
 * Scroll-reveal animation using IntersectionObserver.
 * Elements with the `.reveal` class fade-up into view when they
 * enter the viewport. Stagger delays are handled via CSS nth-child rules.
 */
function initRevealAnimations(): void {
  const reveals = document.querySelectorAll<HTMLElement>(".reveal");
  if (reveals.length === 0) return;

  const observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          entry.target.classList.add("is-visible");
          observer.unobserve(entry.target);
        }
      }
    },
    {
      threshold: 0.15,
      rootMargin: "0px 0px -40px 0px",
    },
  );

  for (const el of reveals) {
    observer.observe(el);
  }
}

// Initialize when DOM is ready
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initRevealAnimations);
} else {
  initRevealAnimations();
}
