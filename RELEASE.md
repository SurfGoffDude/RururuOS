# RururuOS Release Process

## Version Scheme

RururuOS uses semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes, major features
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, security updates

## Release Types

### Alpha (Internal Testing)

- Tag: `vX.Y.Z-alpha.N`
- Audience: Core developers
- Quality: May have known issues
- Duration: 2-4 weeks

### Beta (Community Testing)

- Tag: `vX.Y.Z-beta.N`
- Audience: Community testers
- Quality: Feature complete, may have bugs
- Duration: 4-6 weeks

### Release Candidate (RC)

- Tag: `vX.Y.Z-rc.N`
- Audience: Early adopters
- Quality: Production ready candidate
- Duration: 1-2 weeks

### Stable

- Tag: `vX.Y.Z`
- Audience: All users
- Quality: Production ready

## Release Checklist

### Pre-Release

- [ ] All tests passing
- [ ] No critical issues open
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped in Cargo.toml
- [ ] ISO builds successfully
- [ ] ARM64 images built
- [ ] Hardware compatibility verified

### Release

- [ ] Create git tag
- [ ] Push tag to trigger CI
- [ ] Verify CI artifacts
- [ ] Create GitHub release
- [ ] Upload checksums
- [ ] Update website
- [ ] Announce on social media

### Post-Release

- [ ] Monitor issue tracker
- [ ] Respond to feedback
- [ ] Plan hotfix if needed

## Creating a Release

### 1. Update Version

```bash
# Update version in root Cargo.toml
# Update version in all package Cargo.toml
```

### 2. Update Changelog

```bash
# Add entry to CHANGELOG.md
```

### 3. Create Tag

```bash
git tag -a v1.0.0 -m "RururuOS 1.0.0"
git push origin v1.0.0
```

### 4. CI Builds

GitHub Actions will automatically:
- Build x86_64 packages
- Build ARM64 packages
- Build ISO images
- Create GitHub release
- Upload artifacts

### 5. Verify Release

- Download and test ISO
- Verify checksums
- Test installation

## Hotfix Process

For critical bugs in stable releases:

```bash
# Create hotfix branch from tag
git checkout -b hotfix/1.0.1 v1.0.0

# Apply fix
# ...

# Tag and release
git tag -a v1.0.1 -m "Hotfix: description"
git push origin v1.0.1
```

## Long-Term Support (LTS)

Every major version receives:
- 1 year of active development
- 2 years of security updates

## Release Schedule

| Version | Type | Target Date |
|---------|------|-------------|
| 1.0.0-alpha.1 | Alpha | TBD |
| 1.0.0-beta.1 | Beta | TBD |
| 1.0.0-rc.1 | RC | TBD |
| 1.0.0 | Stable | TBD |
