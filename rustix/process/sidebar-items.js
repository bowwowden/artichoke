initSidebarItems({"constant":[["EXIT_FAILURE","`EXIT_FAILURE` for use with `exit`."],["EXIT_SIGNALED_SIGABRT","The exit status used by a process terminated with `SIGABRT` signal."],["EXIT_SUCCESS","`EXIT_SUCCESS` for use with `exit`."]],"enum":[["MembarrierCommand","A command for use with `membarrier` and `membarrier_cpu`."],["Resource","A resource value for use with `getrlimit`, `setrlimit`, and `prlimit`."],["Signal","A signal number for use with `kill_process` and `kill_process_group`."]],"fn":[["chdir","`chdir(path)`—Change the working directory."],["clock_ticks_per_second","`sysconf(_SC_CLK_TCK)`—Returns the process’ clock ticks per second."],["fchdir","`fchdir(fd)`—Change the working directory."],["getcwd","`getcwd()`"],["getegid","`getegid()`—Returns the process’ effective group ID."],["geteuid","`geteuid()`—Returns the process’ effective user ID."],["getgid","`getgid()`—Returns the process’ real group ID."],["getpid","`getpid()`—Returns the process’ ID."],["getppid","`getppid()`—Returns the parent process’ ID."],["getpriority_pgrp","`getpriority(PRIO_PGRP, gid)`—Get the scheduling priority of the given process group."],["getpriority_process","`getpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given process."],["getpriority_user","`getpriority(PRIO_USER, uid)`—Get the scheduling priority of the given user."],["getrlimit","`getrlimit(resource)`—Get a process resource limit value."],["getuid","`getuid()`—Returns the process’ real user ID."],["kill_current_process_group","`kill(0, sig)`—Sends a signal to all processes in the current process group."],["kill_process","`kill(pid, sig)`—Sends a signal to a process."],["kill_process_group","`kill(-pid, sig)`—Sends a signal to all processes in a process group."],["linux_execfn","`getauxval(AT_EXECFN)`—Returns the Linux “execfn” string."],["linux_hwcap","`(getauxval(AT_HWCAP), getauxval(AT_HWCAP2)`—Returns the Linux “hwcap” data."],["membarrier","`membarrier(cmd, 0, 0)`—Perform a memory barrier."],["membarrier_cpu","`membarrier(cmd, MEMBARRIER_CMD_FLAG_CPU, cpu)`—Perform a memory barrier with a specific CPU."],["membarrier_query","`membarrier(MEMBARRIER_CMD_QUERY, 0, 0)`—Query the supported `membarrier` commands."],["nice","`nice()`—Adjust the scheduling priority of the current process."],["page_size","`sysconf(_SC_PAGESIZE)`—Returns the process’ page size."],["prlimit","`prlimit(pid, resource, new)`—Get and set a process resource limit value."],["sched_getaffinity","`sched_getaffinity(pid)`—Get a thread’s CPU affinity mask."],["sched_setaffinity","`sched_setaffinity(pid, cpuset)`—Set a thread’s CPU affinity mask."],["sched_yield","`sched_yield()`—Hints to the OS that other processes should run."],["setpriority_pgrp","`setpriority(PRIO_PGRP, pgid)`—Get the scheduling priority of the given process group."],["setpriority_process","`setpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given process."],["setpriority_user","`setpriority(PRIO_USER, uid)`—Get the scheduling priority of the given user."],["setrlimit","`setrlimit(resource, new)`—Set a process resource limit value."],["setsid","`setsid()`—Create a new session."],["uname","`uname()`—Returns high-level information about the runtime OS and hardware."],["wait","`wait(waitopts)`—Wait for any of the children of calling process to change state."],["waitpid","`waitpid(pid, waitopts)`—Wait for a specific process to change state."]],"struct":[["CpuSet","`CpuSet` represents a bit-mask of CPUs."],["Cpuid","A Linux CPU ID."],["Gid","`gid_t`—A Unix group ID."],["MembarrierQuery","A result from [`membarrier_query`]."],["Pid","`pid_t`—A non-zero Unix process ID."],["Rlimit","`struct rlimit`—Current and maximum values used in [`getrlimit`], [`setrlimit`], and [`prlimit`]."],["Uid","`uid_t`—A Unix user ID."],["Uname","`struct utsname`—Return type for [`uname`]."],["WaitOptions","Options for modifying the behavior of wait/waitpid"],["WaitStatus","the status of the child processes the caller waited on"]],"type":[["RawGid","A group identifier as a raw integer."],["RawNonZeroPid","A non-zero process identifier as a raw non-zero integer."],["RawPid","A process identifier as a raw integer."],["RawUid","A user identifier as a raw integer."]]});