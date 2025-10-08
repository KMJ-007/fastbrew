# Fastbrew Development Checklist

- [ ] **Transparent Documentation**

  - [ ] Include README with clear goals (e.g., "Faster than Homebrew, bottle-focused, beta").

  - [ ] State limitations (e.g., "v0.1 supports bottles, no casks; source builds slower").

  - [ ] Explain trade-offs clearly(e.g., "Speed prioritized, but compiles for older macOS").

- [ ] **Stability and Atomic Operations**

  - [ ] Implement atomic installs/upgrades (all or nothing, no partial states).

  - [ ] Add snapshot support for rollbacks (JSON manifests of package state).

  - [ ] Test rollbacks for failed installs (e.g., network errors, disk full, system crash).

  - [ ] Support dependency isolation (scopes: global, project-specific).

- [ ] **Robust Error Handling**

  - [ ] Check preconditions (disk space, permissions, Xcode CLT) before operations.

  - [ ] Provide actionable error messages (e.g., "PostgreSQL running? Run `fastbrew services stop postgresql`").

  - [ ] Use fuzzy matching for typos (e.g., `neovm` suggests `neovim`).

  - [ ] Log errors to SQLite for `fastbrew log` command.

- [ ] **Safe Design Choices**

  - [ ] Default to user-space prefix (`/opt/fastbrew`, not `/usr/local`).

  - [ ] Verify bottle checksums and signatures for security.

  - [ ] Avoid risky low-level calls (e.g., direct `launchctl`) without fallbacks.

  - [ ] Support older macOS (e.g., Monterey) via bottle archives, warn on source builds.

- [ ] **Performance Optimizations**

  - [ ] Use SQLite for metadata caching (instant queries, no Ruby parsing).

  - [ ] Implement tarball-based metadata updates (like pacman, not Git pulls).

  - [ ] Enable parallel downloads/extractions (Rust Tokio, improve on Homebrew v4.6.0).

  - [ ] Show progress bars for downloads/installs

- [ ] **Security and Misuse Prevention**

  - [ ] Integrate NVD CVE checks in `fastbrew doctor` and at the time of install also, want to make it work offline also(e.g., flag vulnerable openssl).

  - [ ] Avoid sudo; run in user space to prevent PATH exploits.

  - [ ] Add warnings for risky flags (e.g., `--force`).

  - [ ] Validate tap/formula metadata to prevent untrusted code execution.

- [ ] **Testing and Benchmarks**

  - [ ] Benchmark vs. Homebrew, MacPorts, Sapphire (e.g., `install neovim llvm` timings).

  - [ ] Test macOS versions (Monterey to Tahoe) and architectures (ARM, Intel).

  - [ ] Test edge cases: no network, low disk, broken bottles, macOS upgrades.

  - [ ] Publish benchmark results in README.

- [ ] **Community Engagement**

  - [ ] Open-source MVP early (GitHub, share on r/rust, r/MacOS).

  - [ ] Respond to feedback constructively and improve things.

  - [ ] Provide clear roadmap (e.g., “v0.2: casks, v1.0: Homebrew parity”).

- [ ] **macOS-Specific Compatibility**

  - [ ] Ensure compatibility with macOS 26 (Tahoe) and older (e.g., Monterey).

  - [ ] Reuse Homebrew’s JSON API and bottle archives for metadata and installs.

  - [ ] Avoid bundling libraries that break older macOS (e.g., libz issues).
