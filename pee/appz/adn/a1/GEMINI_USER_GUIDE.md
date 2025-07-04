# Aderyn Gemini AI Integration - User Guide

## Overview

Aderyn now includes AI-powered security analysis using Google's Gemini API. This feature flattens your Solidity contracts and sends them to Gemini for comprehensive security analysis, providing insights beyond traditional pattern-based detection.

## Quick Start

### Prerequisites
- Google Cloud account with Gemini API access
- Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))
- Solidity project with .sol files

### Basic Usage

```bash
# Analyze current directory with default settings
aderyn gemini

# Specify custom output file
aderyn gemini -o my_security_analysis.md

# Get help
aderyn gemini --help
```

## Step-by-Step Guide

### 1. Navigate to Your Project
```bash
cd /path/to/your/solidity/project
```

### 2. Run Gemini Analysis
```bash
aderyn gemini
```

### 3. Interactive Setup
The tool will prompt you for:

#### API Key Input
```
Enter your Gemini API key: [hidden input]
```
- Your API key is securely entered (not visible on screen)
- No credentials are stored or logged

#### Model Selection
```
Available Gemini models:
1. gemini-1.5-flash (Fast, cost-effective)
2. gemini-1.5-pro (More capable, higher cost)  
3. gemini-1.0-pro (Legacy model)

Select model (1-3) [default: 1]:
```

#### Cost Confirmation
```
Estimated cost: ~$0.05
Continue with analysis? (y/N):
```

### 4. Analysis Process
```
Starting Gemini AI Analysis...
===============================
Flattening contracts from: ./
Found and flattened contracts for project: MyDeFiProject
Total code size: 15847 characters
Selected model: gemini-1.5-flash
Estimated cost: ~$0.05

Sending code to Gemini AI for analysis...
This may take a few moments...

Analysis complete!
Report saved to: gemini_analysis.md
```

## Understanding the Analysis Report

### Report Structure

The generated markdown report includes:

#### Header Information
- **Project Name**: Auto-detected from package.json/foundry.toml
- **Analysis Date**: Timestamp of analysis
- **AI Model**: Gemini model used
- **Analyzer**: Aderyn + Gemini AI

#### Security Analysis Sections

1. **Critical Security Issues**
   - High-severity vulnerabilities
   - Potential for fund loss
   - Immediate action required

2. **Medium Risk Issues**
   - Security concerns to address
   - Potential attack vectors
   - Recommended fixes

3. **Low Risk Issues**
   - Code quality improvements
   - Best practice violations
   - Style and maintainability

4. **Gas Optimization Opportunities**
   - Ways to reduce gas consumption
   - Efficiency improvements
   - Cost savings

5. **Code Quality Assessment**
   - Overall structure evaluation
   - Maintainability analysis
   - Architecture recommendations

6. **Specific Recommendations**
   - Actionable steps
   - Implementation guidance
   - Priority suggestions

#### Disclaimer and Limitations
- AI analysis limitations
- Manual review recommendations
- Comprehensive security practices

## Advanced Features

### Cost Estimation

The tool provides upfront cost estimates based on:
- Code length (character count)
- Selected model pricing
- Estimated token usage

**Model Pricing (approximate)**:
- `gemini-1.5-flash`: $0.15 per 1M tokens (most cost-effective)
- `gemini-1.5-pro`: $3.50 per 1M tokens (most capable)
- `gemini-1.0-pro`: $0.50 per 1M tokens (legacy)

### Project Detection

Automatic project name detection from:
1. `package.json` (name field)
2. `foundry.toml` (name field)
3. Directory name (fallback)

### Smart Contract Flattening

The tool intelligently:
- Recursively scans for .sol files
- Skips non-source directories (node_modules, .git, artifacts, cache)
- Combines all contracts with file headers
- Preserves file structure information

## Configuration Options

### Command Line Options

```bash
aderyn gemini [OPTIONS]

Options:
  -o, --output <OUTPUT>    Output file for the analysis report [default: gemini_analysis.md]
  -h, --help              Print help
```

### Environment Variables

You can set environment variables for convenience:
```bash
export GEMINI_API_KEY="your-api-key-here"
export GEMINI_DEFAULT_MODEL="gemini-1.5-flash"
```

## Best Practices

### 1. Review Before Production
- Always manually review AI analysis
- Conduct additional security audits
- Test thoroughly before deployment

### 2. Cost Management
- Start with smaller contracts to understand costs
- Use gemini-1.5-flash for initial analysis
- Upgrade to gemini-1.5-pro for complex projects

### 3. Iterative Analysis
- Run analysis during development
- Re-analyze after significant changes
- Compare reports across versions

### 4. Combine with Traditional Analysis
- Use alongside Aderyn's pattern-based detectors
- Cross-reference findings
- Validate AI suggestions with manual review

## Troubleshooting

### Common Issues

#### "No Solidity files found"
- Ensure you're in the correct directory
- Check that .sol files exist in the project
- Verify file permissions

#### "API key cannot be empty"
- Ensure you enter a valid Gemini API key
- Check API key permissions in Google Cloud Console

#### "Gemini API error"
- Verify API key is correct and active
- Check internet connectivity
- Ensure sufficient API quota

#### "Error saving report"
- Check file permissions in output directory
- Ensure sufficient disk space
- Verify output path is valid

### Getting Help

1. **Check the error message** - Most issues have descriptive error messages
2. **Verify API setup** - Ensure your Gemini API key is valid
3. **Test with small project** - Try with a simple contract first
4. **Check network connectivity** - Ensure internet access for API calls

## Security and Privacy

### Data Handling
- Code is sent to Google's Gemini API for analysis
- No API keys are stored locally
- No analysis results are cached by Aderyn

### Privacy Considerations
- Consider using local models for sensitive code
- Review Google's data handling policies
- Use test contracts for initial evaluation

### API Key Security
- Never commit API keys to version control
- Use environment variables or secure input
- Rotate keys regularly
- Monitor API usage in Google Cloud Console

## Integration with Development Workflow

### CI/CD Integration
```yaml
# Example GitHub Actions workflow
- name: Run Aderyn Gemini Analysis
  env:
    GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
  run: |
    aderyn gemini -o security-analysis.md
    # Upload report as artifact
```

### Pre-deployment Checklist
- [ ] Run Aderyn pattern-based analysis
- [ ] Run Gemini AI analysis
- [ ] Manual code review
- [ ] Security audit (for high-value contracts)
- [ ] Comprehensive testing
- [ ] Gradual deployment with monitoring

## Comparison with Traditional Analysis

| Feature | Pattern-Based | AI-Enhanced |
|---------|---------------|-------------|
| **Speed** | Very Fast | Moderate |
| **Cost** | Free | API costs |
| **Accuracy** | High (known patterns) | Variable |
| **Coverage** | Known vulnerabilities | Broader analysis |
| **Context** | Limited | Business logic aware |
| **Explanations** | Generic | Contextual |
| **Novel Issues** | Limited | Better detection |

## Future Enhancements

Planned features:
- Local model support for offline analysis
- Custom prompt templates
- Integration with existing Aderyn detectors
- Batch processing for multiple projects
- Cost optimization features
- Configuration file support

## Support and Feedback

- **GitHub Issues**: Report bugs and feature requests
- **Documentation**: Check official Aderyn docs
- **Community**: Join discussions on Discord/Twitter
- **Contributing**: Help improve the AI integration

---

**Remember**: AI analysis is a powerful tool but should complement, not replace, traditional security practices. Always conduct thorough manual reviews and testing before deploying smart contracts to production.