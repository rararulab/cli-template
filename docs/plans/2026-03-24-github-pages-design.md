# GitHub Pages Landing Page

## Goal
Create a public-facing landing page for cli-template deployed via GitHub Pages, using jitter.video-inspired scroll animations and the @rararulab/rara design system.

## Approach
Vite + vanilla TypeScript in `web/` directory. Design tokens inlined as CSS custom properties. IntersectionObserver for scroll-triggered reveal animations. Deployed via shared deploy-pages.yml workflow from rararulab/workflows.

## Affected Crates/Modules
| Area | What changes | Why |
|------|-------------|-----|
| web/ | New directory with Vite project | Landing page frontend |
| .github/workflows/pages.yml | New workflow | GitHub Pages deployment |
| docs/plans/ | Design doc | Architecture record |

## Key Decisions
- Vanilla TS over React: consistency with rararulab-site, minimal overhead
- Inlined tokens over npm dependency: avoids version drift for a simple landing page
- Progressive enhancement: content visible without JS
- No parallax: reduces complexity, avoids mobile jank

## Implementation Steps
1. Scaffold web/ with Vite + TypeScript
2. Create CSS with design tokens
3. Build HTML structure
4. Implement scroll animations
5. Add GitHub Actions workflow
6. Verify build
