
pub enum PcaChannelError {
    InvalidChannel
}

pub struct PcaChannel {
    pub channel: usize,
    pub min_pwm: u16,
    pub max_pwm: u16,
    pub channel_type: PcaChannelType
} 
impl PcaChannel {
    pub fn new(channel: usize, channel_type: PcaChannelType) ->Result<PcaChannel,  PcaChannelError> {
        if channel > 15 {
            return Err(PcaChannelError::InvalidChannel)
        }
        match channel_type {
            PcaChannelType::SG90 => Ok(PcaChannel {
                channel,
                min_pwm: 409,
                max_pwm: 2047,
                channel_type
            }),
            PcaChannelType::HOBBYWING => Ok(PcaChannel {
                channel, 
                min_pwm: 0, // change later
                max_pwm: 0, // change later
                channel_type
            })
        }
    }
}

#[derive(Copy, Clone)]
pub enum PcaChannelType {
    SG90,
    HOBBYWING
}