import os


def linux_browser_apps_to_cmd(*apps: str) -> str:
    """Create chrome version command from browser app names.
    Result command example:
        chromium --version || chromium-browser --version
    """
    ignore_errors_cmd_part = " 2>/dev/null"
    return " || ".join(
        list(map(lambda i: f"{i} --version{ignore_errors_cmd_part}", apps))
    )


# google-chrome --version || google-chrome-stable --version

print(linux_browser_apps_to_cmd("google-chrome", "google-chrome-stable"))

