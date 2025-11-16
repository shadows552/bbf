package container.authz

import future.keywords.if
import future.keywords.in

# Default deny for container operations
default allow := false

# Allow read-only operations
allow if {
    input.operation == "read"
}

# Allow write operations to specific directories only
allow if {
    input.operation == "write"
    allowed_write_path
}

# Deny writes to sensitive system directories
allowed_write_path if {
    not startswith(input.path, "/bin")
    not startswith(input.path, "/sbin")
    not startswith(input.path, "/etc")
    not startswith(input.path, "/usr/bin")
    not startswith(input.path, "/usr/sbin")
    not startswith(input.path, "/root")
}

# Allow writes to application directories
allowed_write_path if {
    startswith(input.path, "/app")
}

allowed_write_path if {
    startswith(input.path, "/tmp")
}

allowed_write_path if {
    startswith(input.path, "/var/log")
}

# Network policies
allow_network if {
    input.operation == "connect"
    allowed_destination
}

# Allowed network destinations
allowed_destination if {
    # Allow Solana RPC
    contains(input.destination, "solana.com")
}

allowed_destination if {
    # Allow internal services
    input.destination in [
        "loki:3100",
        "opa:8181",
        "backend:3000",
        "frontend:8080"
    ]
}

# Process execution policies
allow_exec if {
    input.operation == "exec"
    allowed_process
}

# Allowed processes in containers
allowed_process if {
    input.process in [
        "node",
        "nginx",
        "/usr/local/bin/node",
        "/usr/sbin/nginx"
    ]
}

# Deny shell access (for runtime security)
deny_shell if {
    input.process in ["sh", "bash", "ash", "zsh", "csh"]
}
