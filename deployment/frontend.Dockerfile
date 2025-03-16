# Stage 1: Build the React app with Vite
FROM node:18-alpine AS builder
LABEL stage=builder

WORKDIR /app

# Install dependencies
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Copy all project files
COPY . .

# Build the app for production
RUN yarn build

# Stage 2: Serve with Caddy
FROM caddy:alpine AS final
LABEL stage=final

# Copy built app to Caddy's web root
COPY --from=builder /app/dist /usr/share/caddy

# Copy the Caddyfile into the container
COPY Caddyfile /etc/caddy/Caddyfile

# Expose Vite's default port
EXPOSE 5173

# Run Caddy
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile"]
