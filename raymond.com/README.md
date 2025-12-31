# Raymond Progress Tracking Site

A personal progress tracking site for the Raymond embedded Rust project, featuring a subtle Robotic/Cyberpunk + Mission Control aesthetic.

---

## Tech Stack

| Component  | Technology            | Notes                              |
| ---------- | --------------------- | ---------------------------------- |
| Framework  | **Next.js 16**        | App Router                         |
| Styling    | **TailwindCSS 4**     | Already installed                  |
| UI         | **shadcn/ui**         | Default style                      |
| Animations | **Framer Motion**     | To be installed                    |
| CMS/Blog   | **Outstatic**         | Git-based, lives in repo           |
| Database   | **PostgreSQL**        | Docker (local) / Neon (production) |
| ORM        | **Prisma**            | Type-safe queries                  |
| Auth       | ENV-based credentials | Simple server-side check           |
| Deployment | **Vercel**            | Automatic from GitHub              |

---

## Design System

### Typography

| Usage                             | Font          |
| --------------------------------- | ------------- |
| **Sans-serif** (headings, body)   | **Unbounded** |
| **Monospace** (code, data, stats) | **Kode Mono** |

> **Important:** Use Kode Mono sparingly - only for code snippets, data values, and status indicators.

### Color Palette

| Token             | Hex                      | Usage                                |
| ----------------- | ------------------------ | ------------------------------------ |
| `--bg-dark`       | `#0e1116`                | Primary background (rich black)      |
| `--bg-surface`    | `#161b22`                | Cards, panels, elevated surfaces     |
| `--accent-cyan`   | `#00d4ff`                | Primary accent, interactive elements |
| `--accent-amber`  | `#ffb347`                | Warnings, highlights, progress bars  |
| `--accent-purple` | `#a855f7`                | Secondary accent, tags, badges       |
| `--text-primary`  | `#e6edf3`                | Main text                            |
| `--text-muted`    | `#7d8590`                | Secondary text, captions             |
| `--border-glow`   | `rgba(0, 212, 255, 0.3)` | Subtle neon borders                  |

### Visual Elements

- **Borders**: Thin (1px) with subtle glow on hover
- **Cards**: Dark surfaces with slight transparency
- **Shadows**: Colored glow effects instead of traditional shadows
- **Interactions**: Glow intensifies on hover

---

## Local Development

### Prerequisites

- Docker and Docker Compose

### Setup

```bash
# Start local PostgreSQL
docker compose up -d

# Install dependencies
npm install

# Generate Prisma client and push schema
npx prisma generate
npx prisma db push

# Run development server
npm run dev
```

### Environment Variables

```env
# .env.local (local development)
DATABASE_URL="postgresql://raymond:raymond123@localhost:5432/raymond_dev"
ADMIN_USERNAME="admin"
ADMIN_PASSWORD="admin123"
```

---

## Phase 1: Foundation (MVP)

### 1.1 Local Database Setup

- `docker-compose.yml` - PostgreSQL 16 Alpine
- `.env.local` - Local database credentials

### 1.2 Design System

- `tailwind.config.ts` - Fonts (Unbounded, Kode Mono), colors
- `app/globals.css` - CSS variables, dark theme, glow utilities
- `app/layout.tsx` - Google Fonts import

### 1.3 shadcn/ui Setup

- Initialize with default style
- Add core components: button, input, card, form, sidebar

### 1.4 Prisma Schema

- `prisma/schema.prisma` - StatusUpdate, Milestone models
- `lib/db.ts` - Prisma client singleton

### 1.5 Authentication

- `lib/auth.ts` - Credential verification
- `app/admin/login/page.tsx` - Login form
- `app/admin/layout.tsx` - Protected layout with sidebar

### 1.6 Admin Dashboard

- `app/admin/page.tsx` - Overview
- `app/admin/status/page.tsx` - Status update CRUD
- `app/admin/milestones/page.tsx` - Milestone CRUD
- API routes for each

### 1.7 Public Homepage

- Single scrollable page
- Hero section with project name
- Overall progress indicator
- Latest status panel
- Upcoming milestones

---

## Phase 2-5: To Be Detailed

Phases 2-5 will be planned in detail after Phase 1 completion:

- **Phase 2**: Blog (Outstatic) + Resource Library
- **Phase 3**: Focus Widget + Comments/Ideas
- **Phase 4**: Animations + Gallery + Mobile Polish
- **Phase 5**: GitHub Integration (nice to have)
