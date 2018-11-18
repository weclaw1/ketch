use vulkano::instance::{PhysicalDevice, QueueFamily};
use vulkano::swapchain::Surface;
use vulkano::device::{Queue, QueuesIter};
use winit::Window;

use log::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Queues {
    graphics_queue: Arc<Queue>,
    compute_queue: Arc<Queue>,
    transfer_queue: Arc<Queue>,
}

impl Queues {
    pub fn new(queues: QueuesIter) -> Self {
        let mut graphic_queue = None;
        let mut compute_queue = None;
        let mut transfer_queue = None;

        for queue in queues{
            if queue.family().supports_graphics() && graphic_queue.is_none() {
                graphic_queue = Some(queue);
                continue;
            }

            if queue.family().supports_compute() && compute_queue.is_none() {
                compute_queue = Some(queue);
                continue;
            }

            if !queue.family().supports_transfers() && transfer_queue.is_none(){
                transfer_queue = Some(queue);
                continue;
            }
        }

        //Check the queues
        if graphic_queue.is_none(){
            panic!("No graphics queue found");
        }

        if compute_queue.is_none(){
            warn!("No compute queue found, using graphics queue", );
            compute_queue = graphic_queue.clone();
        }

        //Currently always using compute queue since multi queue sync is not implemented for vulkano atm.
        transfer_queue = compute_queue.clone();

        if transfer_queue.is_none(){
            warn!("No transfer queue found, using compute", );
            transfer_queue = compute_queue.clone();
        }

        Queues{
            graphics_queue: graphic_queue.expect("Failed to find graphics queue"),
            compute_queue: compute_queue.expect("Failed to find compute queue"),
            transfer_queue: transfer_queue.expect("Failed to find transfer queue"),
        }
    }

    pub fn graphics_queue(&self) -> Arc<Queue> {
        self.graphics_queue.clone()
    }

    pub fn compute_queue(&self) -> Arc<Queue> {
        self.compute_queue.clone()
    }

    pub fn transfer_queue(&self) -> Arc<Queue> {
        self.transfer_queue.clone()
    }
}

pub fn find_queues<'a>(physical_device: &PhysicalDevice<'a>, surface: &Surface<Window>) -> Vec<(QueueFamily<'a>, f32)>{
    
    let mut queue_collection = Vec::new();

    let mut has_graphics = false;
    let mut has_compute = false;
    let mut has_transfer = false;

    info!("QUEUE INFO:\n=========");

    for queue in physical_device.queue_families(){
        info!("Queue {}, graphics: {}, compute: {}, count: {}",
            queue.id(),
            queue.supports_graphics(),
            queue.supports_compute(),
            queue.queues_count()
        );

        if queue.supports_graphics() && surface.is_supported(queue).unwrap_or(false) && !has_graphics {
            queue_collection.push((queue, 1.0));
            has_graphics = true
        }

        if queue.supports_compute() && !queue.supports_graphics() && !has_compute{
            queue_collection.push((queue, 0.75));
            has_compute = true;
        }

        if queue.supports_transfers() && !queue.supports_compute() && !queue.supports_graphics() && !has_transfer{
            queue_collection.push((queue, 0.5));
            has_transfer = true;
        }

    }

    info!("=========");

    info!("Found queues: graphics: {}, compute: {}, transfer: {}", has_graphics, has_compute, has_transfer);

    queue_collection
}
