use gst::{prelude::*, Pipeline};
use gstreamer as gst;
use nmos_model::resource;
use nmos_node::Node;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn create_pipeline() -> Result<Pipeline, Box<dyn std::error::Error>> {
    // Create VP8 RTP video test pipeline
    let pipeline = gst::Pipeline::default();

    let src = gst::ElementFactory::make("videotestsrc").build()?;
    let q1 = gst::ElementFactory::make("queue").build()?;
    let enc = gst::ElementFactory::make("vp8enc").build()?;
    let q2 = gst::ElementFactory::make("queue").build()?;
    let pay = gst::ElementFactory::make("rtpvp8pay").build()?;
    let rtpbin = gst::ElementFactory::make("rtpbin").build()?;
    let sink = gst::ElementFactory::make("udpsink").build()?;

    pipeline.add_many(&[&src, &q1, &enc, &q2, &pay, &rtpbin, &sink])?;

    src.link(&q1)?;
    q1.link(&enc)?;
    enc.link(&pay)?;
    pay.link(&q2)?;

    let srcpad = q2.static_pad("src").unwrap();
    let sinkpad = rtpbin.request_pad_simple("send_rtp_sink_0").unwrap();
    srcpad.link(&sinkpad)?;

    let srcpad = rtpbin.static_pad("send_rtp_src_0").unwrap();
    let sinkpad = sink.static_pad("sink").unwrap();
    srcpad.link(&sinkpad)?;

    // Encoder properties
    enc.set_property("keyframe-max-dist", 30i32);
    enc.set_property("deadline", 1i64);

    // UDP sink properties
    sink.set_property("host", "0.0.0.0");
    sink.set_property("sync", true);

    Ok(pipeline)
}

fn create_node() -> Node {
    // Create NMOS node
    let node = resource::Node::builder("GStreamer test node", "http://127.0.0.1:3000/test").build();
    let device = resource::Device::builder(
        "GStreamer test device",
        &node,
        resource::DeviceType::Generic,
    )
    .build();

    // Create source and flow for video
    let source =
        resource::Source::builder("GStreamer test source", &device, resource::Format::Video)
            .description("SMPTE video test stream")
            .build();
    let flow = resource::Flow::builder("GStreamer VP8 test flow", &source).build();

    // Create sender
    let sender = resource::Sender::builder(
        "GStreamer test sender",
        &device,
        &flow,
        resource::Transport::RtpUnicast,
    )
    .manifest("file:///path/to/sdp/file")
    .build();

    let mut bundle = resource::ResourceBundle::new();
    bundle.insert_node(node);
    bundle.insert_device(device);
    bundle.insert_source(source);
    bundle.insert_flow(flow);
    bundle.insert_sender(sender);

    Node::builder_from_resources(bundle).build()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber");

    // Try to init gstreamer
    gstreamer::init()?;

    // Create pipeline
    let pipeline = create_pipeline()?;

    // Create NMOS node
    let node = create_node();

    // Start pipeline on separate thread
    std::thread::spawn(move || {
        // Get bus
        let bus = pipeline.bus().expect("Pipeline without bus!");

        // Start pipeline
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set Playing state");

        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => panic!("EOS in test stream"),
                MessageView::Error(err) => {
                    eprintln!("Error: {} {:?}", err.error(), err.debug());
                    break;
                }
                MessageView::StateChanged(state) => {
                    if let Some(element) = msg.src() {
                        if element == &pipeline && state.current() == gst::State::Playing {
                            println!("Playing")
                        }
                    }
                }
                MessageView::StreamStatus(status) => {
                    if let Some(element) = msg.src() {
                        if element == &pipeline {
                            println!("{:?}", status);
                        }
                    }
                }
                _ => {}
            }
        }

        pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set Null state");
    });

    // Run node
    node.start_blocking()?;

    Ok(())
}
