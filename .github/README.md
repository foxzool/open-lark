# GitHub Workflows

This directory contains the GitHub Actions workflows and templates for the open-lark project.

## 🔄 Workflows

### CI Workflow (`ci.yml`)

Runs on every push and pull request:
- ✅ Format checking (`cargo fmt`)
- 🔍 Linting (`cargo clippy`) 
- 🧪 Tests (`cargo test`)
- 📚 Documentation generation (`cargo doc`)

### Release Workflow (`release.yml`)

Triggers when a version tag is pushed (e.g., `v0.4.0`):

1. **Validation**: Runs all CI checks
2. **GitHub Release**: Creates a GitHub release with changelog
3. **Crates.io**: Publishes to crates.io registry
4. **Documentation**: Updates GitHub Pages docs

## 🚀 Creating a Release

### Prerequisites

Before creating a release, ensure you have:

1. **Crates.io API Token**: 
   - Generate at [crates.io/me](https://crates.io/me)
   - Add as repository secret: `CRATES_IO_TOKEN`

2. **GitHub Token**: 
   - Automatically provided as `GITHUB_TOKEN`
   - No additional setup needed

### Manual Release Process

1. **Update version** in `Cargo.toml`
2. **Update** `CHANGELOG.md` with release notes  
3. **Commit and push** changes
4. **Create and push tag**:
   ```bash
   git tag v0.4.0
   git push origin v0.4.0
   ```

### Automated Release with Just

Use the integrated just command for a guided release:

```bash
just release 0.4.0
```

The command will:
- ✅ Verify working directory is clean
- ✅ Check version consistency
- ✅ Run all pre-release checks
- ✅ Create and push the tag
- 🚀 Trigger automated release pipeline

## 📋 Issue Templates

### Bug Report (`bug_report.yml`)
- Version information
- Environment details
- Reproduction steps
- Code samples and logs

### Feature Request (`feature_request.yml`)
- Feature type classification
- Problem statement
- Proposed solution
- Use case description

## 📝 Pull Request Template

The PR template includes:
- Change type classification
- Testing checklist
- Documentation requirements
- Breaking change assessment

## 🔒 Security

### Security Policy (`SECURITY.md`)
- Supported versions
- Vulnerability reporting process
- Response timeline
- Security best practices

### Repository Secrets

Required secrets for full functionality:

| Secret | Purpose | How to Get |
|--------|---------|------------|
| `CRATES_IO_TOKEN` | Publishing to crates.io | [crates.io/me](https://crates.io/me) |
| `GITHUB_TOKEN` | GitHub API access | Auto-provided |

## 📚 Contributing Guide

See `CONTRIBUTING.md` for detailed contribution guidelines including:
- Development setup
- Code style requirements
- Testing guidelines
- Submission process

## 🎯 Best Practices

### Version Management
- Follow [Semantic Versioning](https://semver.org/)
- Update `CHANGELOG.md` for every release
- Ensure `Cargo.toml` version matches git tags

### Release Notes
- Extract from `CHANGELOG.md` automatically
- Include breaking changes prominently
- Provide migration guidance when needed

### Quality Gates
- All tests must pass
- Code must be formatted (`cargo fmt`)
- No clippy warnings allowed
- Documentation must build successfully

## 🔧 Troubleshooting

### Common Issues

**Release workflow fails with "Version mismatch"**
- Ensure `Cargo.toml` version matches the git tag
- Tag format: `v0.4.0`, Cargo version: `0.4.0`

**Crates.io publish fails**
- Verify `CRATES_IO_TOKEN` secret is set
- Check if version already exists on crates.io
- Ensure all dependencies are published

**Documentation deployment fails**
- Check for documentation build errors
- Verify GitHub Pages is enabled
- Ensure proper permissions for GitHub token

### Manual Recovery

If automated release fails:

1. **Delete the tag** (if needed):
   ```bash
   git tag -d v0.4.0
   git push origin :refs/tags/v0.4.0
   ```

2. **Fix the issue** and try again

3. **Manual crates.io publish**:
   ```bash
   cargo publish --token $CRATES_IO_TOKEN
   ```

## 📞 Support

For workflow-related issues:
- Check [GitHub Actions logs](../../actions)
- Review this documentation
- Create an issue with the `workflow` label