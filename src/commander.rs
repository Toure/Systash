/* Commander module will be responsible for system level tasks
like starting and stopping services, configuring cron schedules, and
generic shell execution.
*/

struct Commander {
    servicename: String,
    filelock: String,
    database: String,
    
}
