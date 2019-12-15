use crate::command::WriteTxPayload;
use crate::config::Configuration;
use crate::device::Device;
use crate::registers::{FifoStatus, ObserveTx, Status};
use crate::standby::StandbyMode;
use core::fmt;

/// Represents **TX Mode** and the associated **TX Settling** and
/// **Standby-II** states
///
/// **" It is important to never keep the nRF24L01 in TX mode for more
/// than 4ms at a time."**
pub struct TxMode<D: Device> {
    device: D,
}

impl<D: Device> fmt::Debug for TxMode<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxMode")
    }
}

impl<D: Device> TxMode<D> {
    /// Relies on everything being set up by `StandbyMode::tx()`, from
    /// which it is called
    pub(crate) fn new(device: D) -> Self {
        TxMode { device }
    }

    /// Disable `CE` so that you can switch into RX mode.
    pub fn standby(mut self) -> Result<StandbyMode<D>, D::Error> {
        self.wait_empty()?;

        Ok(StandbyMode::from_rx_tx(self.device))
    }

    /// Is TX FIFO empty?
    pub fn is_empty(&mut self) -> Result<bool, D::Error> {
        let (_, fifo_status) = self.device.read_register::<FifoStatus>()?;
        Ok(fifo_status.tx_empty())
    }

    /// Is TX FIFO full?
    pub fn is_full(&mut self) -> Result<bool, D::Error> {
        let (_, fifo_status) = self.device.read_register::<FifoStatus>()?;
        Ok(fifo_status.tx_full())
    }

    /// Does the TX FIFO have space?
    pub fn can_send(&mut self) -> Result<bool, D::Error> {
        let full = self.is_full()?;
        Ok(!full)
    }

    /// Send asynchronously
    pub fn send(&mut self, packet: &[u8]) -> Result<(), D::Error> {
        self.device.send_command(&WriteTxPayload::new(packet))?;
        self.device.ce_enable();
        Ok(())
    }

    /// Wait until FX FIFO is empty
    pub fn wait_empty(&mut self) -> Result<(), D::Error> {
        let mut empty = false;
        while !empty {
            let (status, fifo_status) = self.device.read_register::<FifoStatus>()?;
            empty = fifo_status.tx_empty();
            if !empty {
                self.device.ce_enable();
            }

            // TX won't continue while MAX_RT is set
            if status.max_rt() {
                let mut clear = Status(0);
                // Clear TX interrupts
                clear.set_tx_ds(true);
                clear.set_max_rt(true);
                self.device.write_register(clear)?;
            }
        }
        // Can save power now
        self.device.ce_disable();

        Ok(())
    }

    pub fn observe(&mut self) -> Result<ObserveTx, D::Error> {
        let (_, observe_tx) = self.device.read_register()?;
        Ok(observe_tx)
    }
}

impl<D: Device> Configuration for TxMode<D> {
    type Inner = D;
    fn device(&mut self) -> &mut Self::Inner {
        &mut self.device
    }
}
