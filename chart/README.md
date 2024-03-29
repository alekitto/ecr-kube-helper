# sentry-kubernetes

[sentry-kubernetes](https://github.com/alekitto/sentry-kubernetes) is a utility that pushes Kubernetes events to [Sentry](https://sentry.io).

# Installation:

```console
$ helm install oci://ghcr.io/alekitto/sentry-kubernetes-chart/sentry-kubernetes release-name --set sentry.dsn=<your-dsn>
```

## Configuration

The following table lists the configurable parameters of the sentry-kubernetes chart and their default values.

| Parameter                   | Description                                                                                                                 | Default                       |
|-----------------------------|-----------------------------------------------------------------------------------------------------------------------------|-------------------------------|
| `sentry.dsn`                | Sentry dsn                                                                                                                  | Empty                         |
| `sentry.existingSecret`     | The name of the already existing secret containing the DSN                                                                  | Empty                         |
| `sentry.environment`        | Sentry environment                                                                                                          | Empty                         |
| `sentry.release`            | Sentry release                                                                                                              | Empty                         |
| `sentry.logLevel`           | The log level of this application (the sentry reporter)                                                                     | Empty                         |
| `image.repository`          | Container image name                                                                                                        | `getsentry/sentry-kubernetes` |
| `image.tag`                 | Container image tag                                                                                                         | `latest`                      |
| `image.pullPolicy`          | Container pull policy                                                                                                       | `Always`                      |
| `rbac.create`               | If `true`, create and use RBAC resources                                                                                    | `true`                        |
| `serviceAccount.name`       | Service account to be used. If not set and serviceAccount.create is `true`, a name is generated using the fullname template | ``                            |
| `serviceAccount.create`     | If true, create a new service account                                                                                       | `true`                        |
| `priorityClassName`         | pod priorityClassName                                                                                                       | Empty                         |
| `filters.namespaces`        | Only report events from these namespaces                                                                                    | Empty                         |
| `filters.excludeNamespaces` | Do not report events from these namespaces                                                                                  | Empty                         |
| `filters.excludeComponents` | Do not report events from these components                                                                                  | Empty                         |
| `filters.excludeReasons`    | Do not report events with these reasons (error codes)                                                                       | Empty                         |
| `filters.eventLevels`       | Only report events of these levels. "error" events are always reported.                                                     | [ `warning`, `error` ]        |
