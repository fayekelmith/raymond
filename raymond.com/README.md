# Raymond.com

The personal website and digital garden for [Your Name] / Raymond.
Built with Next.js 15 (App Router), Tailwind CSS, Prisma, and PostgreSQL.

## Features

- **Mission Control**: Private admin dashboard to manage content.
- **Focus Widget**: Show visitors what you're currently working on.
- **Ideas System**: Allow visitors to submit feedback/ideas.
- **Resource Library**: Curated list of useful links and tools.
- **Dev Log**: File-based Markdown blog for updates.
- **Cyberpunk Theme**: Custom design with animations and glitch effects.

## Getting Started

1.  **Clone the repo**:

    ```bash
    git clone https://github.com/yourusername/raymond.git
    cd raymond/raymond.com
    ```

2.  **Install dependencies**:

    ```bash
    npm install
    ```

3.  **Environment Setup**:
    Copy `.env.example` to `.env` (or create one) with:

    ```env
    DATABASE_URL="postgresql://user:password@localhost:5432/raymond"
    NEXT_PUBLIC_APP_URL="http://localhost:3000"
    # Auth secrets (if you added auth library, otherwise admin is currently unproctected for demo)
    ```

4.  **Database Setup**:

    ```bash
    npx prisma migrate dev
    ```

5.  **Run Development Server**:
    ```bash
    npm run dev
    ```

## Deployment

This project is optimized for deployment on **Vercel**.

1.  Push your code to a GitHub repository.
2.  Import the project in Vercel.
3.  Add your `DATABASE_URL` to Vercel Environment Variables.
4.  The `postinstall` script (`prisma generate`) will run automatically.
5.  Deploy!

### Scripts

- `npm run dev`: Start dev server
- `npm run build`: Build for production (includes Prisma migration check)
- `npm run lint`: Run ESLint
