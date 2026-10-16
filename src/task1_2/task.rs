#[derive(Debug, Clone, PartialEq)]
pub enum SystemState {
    Uninitialized,
    Ready,
    Navigating { target_x: f64, target_y: f64 },
    Failed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemEvent {
    Initialize,
    StartNavigation { x: f64, y: f64 },
    ReachTarget,
    TriggerFault(String),
    Reset,
}

pub trait StateTransition {
    fn handle_event(&mut self, event: SystemEvent) -> Result<SystemState, String>;
}

pub struct RobotStateMachine {
    pub state: SystemState,
}

impl RobotStateMachine {
    pub fn new() -> Self {
        Self {
            state: SystemState::Uninitialized,
        }
    }
}

impl StateTransition for RobotStateMachine {
    fn handle_event(&mut self, event: SystemEvent) -> Result<SystemState, String> {
        match &self.state {
            SystemState::Uninitialized => {
                self.state = match event {
                    SystemEvent::Initialize => SystemState::Ready,
                    _ => return Err("Cannot execute: Robot is Uninitialized.".to_string()),
                };

                return Ok(SystemState::Ready);
            }
            SystemState::Ready => match event {
                SystemEvent::StartNavigation { x, y } => {
                    self.state = SystemState::Navigating {
                        target_x: x,
                        target_y: y,
                    };
                    return Ok(self.state.clone());
                }
                SystemEvent::TriggerFault(msg) => {
                    self.state = SystemState::Failed(msg);
                    return Ok(self.state.clone());
                }
                _ => return Err("Invalid event for Ready state.".to_string()),
            },
            SystemState::Navigating {
                target_x: _,
                target_y: _,
            } => match event {
                SystemEvent::ReachTarget => {
                    self.state = SystemState::Ready;
                    return Ok(SystemState::Ready);
                }
                SystemEvent::StartNavigation { x, y } => {
                    self.state = SystemState::Navigating {
                        target_x: x,
                        target_y: y,
                    };
                    return Ok(self.state.clone());
                }
                SystemEvent::TriggerFault(msg) => {
                    self.state = SystemState::Failed(msg);
                    return Ok(self.state.clone());
                }
                _ => {
                    return Err("Invalid event for Navigating state.".to_string());
                }
            },
            SystemState::Failed(reason) => match event {
                SystemEvent::Reset => {
                    self.state = SystemState::Uninitialized;
                    return Ok(SystemState::Uninitialized);
                }
                _ => {
                    return Err(format!(
                        "Robot is in Failed state ({reason}). Must Reset first."
                    ));
                }
            },
        }

        // Err("Cannot execute: Robot is Uninitialized.".to_string())
    }
}

pub fn test_task1_2() {
    let mut robot = RobotStateMachine::new();
    // // 1. Trying to move before initialization must fail
    let res = robot.handle_event(SystemEvent::StartNavigation { x: 10.0, y: 5.0 });
    assert!(res.is_err());

    // 2. Initialize
    assert_eq!(
        robot.handle_event(SystemEvent::Initialize),
        Ok(SystemState::Ready)
    );

    // 3. Start Navigation
    let nav_res = robot.handle_event(SystemEvent::StartNavigation { x: 5.2, y: -3.1 });
    assert_eq!(
        nav_res,
        Ok(SystemState::Navigating {
            target_x: 5.2,
            target_y: -3.1
        })
    );

    // 4. Trigger E-Stop / Fault
    let fault_res = robot.handle_event(SystemEvent::TriggerFault("Motor overheat!".to_string()));
    assert_eq!(
        fault_res,
        Ok(SystemState::Failed("Motor overheat!".to_string()))
    );

    // 5. Must reset before using again
    assert!(robot.handle_event(SystemEvent::Initialize).is_err());
    assert_eq!(
        robot.handle_event(SystemEvent::Reset),
        Ok(SystemState::Uninitialized)
    );

    robot.state = SystemState::Navigating {
        target_x: 10.0,
        target_y: 10.0,
    };
    assert_eq!(
        robot.handle_event(SystemEvent::ReachTarget),
        Result::Ok(SystemState::Ready)
    );

    println!("✅ State Machine passed all checks!");
}
