# mihaimateias.com

## Purpose

This directory is the designated local working directory for `https://mihaimateias.com`. Recover or create the site source here, then deploy only through its verified production path.

## Verified production identity

- Public URL: `https://mihaimateias.com/`
- Live title: **Mihai Mateias**
- Live description: `Mihai Mateias: SEO, developer, data scientist.`
- Current homepage: placeholder with a `/projects` link.
- The public domain is proxied by Cloudflare. Its A records resolve to Cloudflare addresses, not the origin.

## Deployment status

- Production is the live `mihaimateias` namespace on the OVH `ovh1` k3s cluster. It runs the `site`, `hp2-web`, and two `cloudflared` deployments.
- Cloudflare Tunnel `8a8a0c15-00b2-48d5-92db-4ad4599d12e8` routes `mihaimateias.com` and `www.mihaimateias.com` to `site`, and `hp2.mihaimateias.com` to `hp2-web`.
- This workspace is the source repository, `https://github.com/cefege/mihaimateias.com`. Flux synchronizes `main` at `deploy/production/`; GitHub Actions builds `ghcr.io/cefege/mihaimateias.com` and pins the new image tag in that Kustomization.

## Working rules

1. Keep all site source and deployment configuration in this repository. Do not rely on server-only edits.
2. Before publishing, run `cargo check --features server` and build the production container locally when Docker is available.
3. Publish through `main`: the image workflow updates `deploy/production/kustomization.yaml`, then Flux deploys it to the OVH cluster. Do not bypass Flux with direct application changes.
4. After Flux reconciles, smoke-test `https://mihaimateias.com/` and every changed route.
5. Keep credentials out of Git. Use documented environment-variable names and a committed `.env.example` only.
