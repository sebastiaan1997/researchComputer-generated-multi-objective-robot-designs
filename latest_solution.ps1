$sol = gci logs | sort LastWriteTime | select -last 1
Copy-Item $sol -Dest ./latest_solution.json