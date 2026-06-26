# Code Simplification & Refactoring TODOs

## High Priority - UI/Event Handlers

- [ ] **Simplify Event Handlers in `cmaes_ui.rs`**
  - Currently using verbose nested pattern matching for event extraction
  - Create `event_helpers.rs` module with utility functions:
    - `get_select_value(ev: &Event) -> Option<String>`
    - `get_input_value(ev: &Event) -> Option<String>`
  - Replace all instances of:
    ```rust
    if let Some(val) = ev.target() {
        if let Some(input) = val.dyn_ref::<HtmlSelectElement>() {
            // ...
        }
    }
    ```
  - With simpler: `event_helpers::get_select_value(&ev)`

- [ ] **Use `event_target_value` from Leptos**
  - Leptos provides built-in helpers in `leptos_dom::helpers`
  - Replace manual event extraction with `event_target_value(&ev)`
  - Affected locations: function select, dimensions input, population size input

## Medium Priority - Code Cleanup

- [ ] **Fix hardcoded Title in `app.rs`**
  - Change from: `<Title text="Leptos + Axum Counter"/>`
  - To: `<Title text="CMA-ES Parameter Optimizer"/>`

- [ ] **Extract Component Props to Type Aliases** (if more components added)
  - Consider defining reusable prop types for section components

- [ ] **Add Comments to Complex State Flows**
  - Document the signal → UI → CSS pattern used for best individual highlighting
  - Explain pause/resume loop logic

## Lower Priority - Nice-to-haves

- [ ] **Create `src/event_helpers.rs` as utility module**
  - Centralize all event extraction logic
  - Make it reusable across components

- [ ] **Type Safety for Event Handlers**
  - Consider creating wrapper types for event handlers
  - Could prevent future bugs with type checking

## Architecture Notes

- Event handling is currently verbose due to web-sys's low-level nature
- Leptos exposes raw DOM APIs without heavy abstraction (good for control, requires verbosity)
- Best pattern so far: signal → conditional class → CSS (used for best individual highlighting)

---

## Enterprise-Grade Learning Features (Future Upgrades)

### Deployment & CI/CD

- [ ] **Add Pre-Deployment Testing**
  - Integrate full test suite into GitHub Actions before Docker build
  - Add integration tests for API endpoints
  - Block deployment if tests fail

- [ ] **Container Image Security Scanning**
  - Add Trivy or Snyk action to scan Docker images for vulnerabilities
  - Fail workflow if critical CVEs detected
  - Generate security reports in GitHub

- [ ] **Multi-Environment Deployment**
  - Create separate workflows for dev → staging → production
  - Require approvals before production deployment
  - Implement blue-green deployment strategy

- [ ] **Automated Rollback Strategy**
  - Keep previous image versions in registry
  - Implement health checks after deployment
  - Auto-rollback to last stable version if health checks fail

### Monitoring & Observability

- [ ] **Application Performance Monitoring (APM)**
  - Integrate with Azure Application Insights
  - Track request latency, error rates, dependencies
  - Set up custom events for optimization metrics (iterations, best fitness)

- [ ] **Structured Logging**
  - Replace simple `log!` with structured logging (e.g., `slog` or `tracing`)
  - Log optimization progress with timestamps and metrics
  - Centralize logs to Azure Log Analytics

- [ ] **Alerting & Thresholds**
  - Set up alerts for deployment failures
  - Monitor app uptime and latency
  - Alert if error rate exceeds threshold

- [ ] **Metrics Dashboard**
  - Create Azure Dashboard showing:
    - Deployment frequency
    - Error rates
    - Response times
    - Custom metrics (optimization runs, success rate)

### Security & Compliance

- [ ] **Secrets Rotation**
  - Integrate with Azure Key Vault for automatic secret rotation
  - Implement key rotation policies

- [ ] **Audit Logging**
  - Log all deployments with timestamp, operator, commit SHA
  - Track configuration changes
  - Maintain audit trail for compliance

- [ ] **Rate Limiting & DDoS Protection**
  - Implement rate limiting on endpoints
  - Use Azure Front Door or WAF for DDoS protection

- [ ] **HTTPS & Security Headers**
  - Enforce HTTPS with HSTS headers
  - Add security headers (CSP, X-Frame-Options, etc.)
  - Use managed certificates (Azure handles this)

### Scalability & Infrastructure

- [ ] **Load Testing & Performance Baseline**
  - Set up load testing pipeline (e.g., k6 or Apache JMeter)
  - Establish performance baselines
  - Run before each release to catch regressions

- [ ] **Containerization Optimization**
  - Reduce Docker image size (slim base images, multi-stage builds)
  - Measure and track image size over time
  - Implement image size budget

- [ ] **Kubernetes Migration Path** (Optional)
  - Document how to migrate from App Service to AKS
  - Create Helm charts for easier deployment
  - Experiment with local Kubernetes (minikube) for testing

- [ ] **Database Integration** (If needed later)
  - Separate database connection strings via Key Vault
  - Implement connection pooling
  - Set up database backups and recovery procedures

### Documentation & Best Practices

- [ ] **Deployment Runbook**
  - Document manual troubleshooting steps
  - Create incident response playbook
  - Document rollback procedures

- [ ] **Architecture Decision Records (ADRs)**
  - Document why Docker was chosen
  - Record Azure Container Registry vs alternatives
  - Track deployment strategy decisions

- [ ] **Cost Optimization**
  - Monitor Azure costs per deployment
  - Identify unused resources
  - Implement auto-shutdown for non-prod environments
