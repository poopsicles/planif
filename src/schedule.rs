use windows::core::BSTR;
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::TaskScheduler::{
    IActionCollection, IRegistrationInfo, ITaskDefinition, ITaskFolder, ITaskService,
    ITaskSettings, ITrigger, ITriggerCollection, TASK_LOGON_INTERACTIVE_TOKEN,
};

#[derive(Debug, PartialEq)]
/// A schedule is created by a [schedule builder](crate::schedule_builder). Once created, the
/// Schedule can be registered with the Windows Task Scheduler.
pub struct Schedule {
    pub(crate) task_folder: ITaskFolder,
    pub(crate) actions: IActionCollection,
    pub(crate) force_start_boundary: bool,
    pub(crate) registration_info: IRegistrationInfo,
    pub(crate) settings: ITaskSettings,
    pub(crate) task_definition: ITaskDefinition,
    pub(crate) task_service: ITaskService,
    pub(crate) trigger: Option<ITrigger>,
    pub(crate) triggers: ITriggerCollection,
    //repetition: IRepetitionPattern,
}

impl Schedule {
    /// Registers the schedule. Flags can be set by using the [TaskCreationFlags](crate::enums::TaskCreationFlags) enum.
    pub fn register(self, task_name: &str, flags: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            self.task_folder.RegisterTaskDefinition(
                &BSTR::from(task_name),
                &self.task_definition,
                flags,
                // TODO allow user to specify creds
                VARIANT::default(),
                VARIANT::default(),
                TASK_LOGON_INTERACTIVE_TOKEN,
                VARIANT::default(),
            )?;
        }

        Ok(())
    }
}
