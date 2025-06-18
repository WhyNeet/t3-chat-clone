# WhyChat
[Deployment on Vercel](https://whychat.vercel.app)
The backend is deployed only in US West, so it might be slow if you are in another region.
### Running locally
1. Set environment variables: 
- OPENROUTER_KEY - you OpenRouter API key.
- MONGODB_URI - MongoDB connection string in the format `mongodb://user:password@host:port`.
- REDIS_URI - Redis connection string with username and password.
- SESSION_SECRET_KEY - a secret key for session token signing.
- SERPER_KEY - Serper API key.
- CHUTES_KEY - Chutes API key.
- KEY_ENCRYPTION_SECRET - a secret key for api key encryption, use `openssl rand -hex 32` to generate it.

2. Docker Compose file is included in the repository, you may use it to run mongodb and redis locally.

3. Run `cargo run --release` with the environment variables set:
```bash
OPENROUTER_KEY="your_key" MONGODB_URI="uri" KEY_ENCRYPTION_SECRET="your_secret" cargo run --release # set other keys too
```

4. Run the frontend: `cd web && pnpm dev`.
