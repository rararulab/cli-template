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
    { threshold: 0.15, rootMargin: "0px 0px -40px 0px" },
  );

  for (const el of reveals) observer.observe(el);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initRevealAnimations);
} else {
  initRevealAnimations();
}
