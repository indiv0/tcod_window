(function() {var implementors = {};
implementors['shader_version'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='shader_version/opengl/enum.OpenGL.html' title='shader_version::opengl::OpenGL'>OpenGL</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='shader_version/glsl/enum.GLSL.html' title='shader_version::glsl::GLSL'>GLSL</a>",];implementors['viewport'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='viewport/struct.Viewport.html' title='viewport::Viewport'>Viewport</a>",];implementors['libc'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/posix01/struct.glob_t.html' title='libc::types::os::common::posix01::glob_t'>glob_t</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/posix01/struct.timeval.html' title='libc::types::os::common::posix01::timeval'>timeval</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/posix01/struct.timespec.html' title='libc::types::os::common::posix01::timespec'>timespec</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/posix01/struct.rlimit.html' title='libc::types::os::common::posix01::rlimit'>rlimit</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd43/struct.rusage.html' title='libc::types::os::common::bsd43::rusage'>rusage</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.sockaddr.html' title='libc::types::os::common::bsd44::sockaddr'>sockaddr</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.sockaddr_storage.html' title='libc::types::os::common::bsd44::sockaddr_storage'>sockaddr_storage</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.sockaddr_in.html' title='libc::types::os::common::bsd44::sockaddr_in'>sockaddr_in</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.in_addr.html' title='libc::types::os::common::bsd44::in_addr'>in_addr</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.sockaddr_in6.html' title='libc::types::os::common::bsd44::sockaddr_in6'>sockaddr_in6</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.in6_addr.html' title='libc::types::os::common::bsd44::in6_addr'>in6_addr</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.ip_mreq.html' title='libc::types::os::common::bsd44::ip_mreq'>ip_mreq</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.ip6_mreq.html' title='libc::types::os::common::bsd44::ip6_mreq'>ip6_mreq</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.addrinfo.html' title='libc::types::os::common::bsd44::addrinfo'>addrinfo</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.sockaddr_un.html' title='libc::types::os::common::bsd44::sockaddr_un'>sockaddr_un</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/common/bsd44/struct.ifaddrs.html' title='libc::types::os::common::bsd44::ifaddrs'>ifaddrs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/arch/posix01/struct.stat.html' title='libc::types::os::arch::posix01::stat'>stat</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/arch/posix01/struct.utimbuf.html' title='libc::types::os::arch::posix01::utimbuf'>utimbuf</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/arch/posix01/struct.pthread_attr_t.html' title='libc::types::os::arch::posix01::pthread_attr_t'>pthread_attr_t</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='libc/types/os/arch/extra/struct.sockaddr_ll.html' title='libc::types::os::arch::extra::sockaddr_ll'>sockaddr_ll</a>",];implementors['tcod_sys'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed1.html' title='tcod_sys::Struct_Unnamed1'>Struct_Unnamed1</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Union_Unnamed2.html' title='tcod_sys::Union_Unnamed2'>Union_Unnamed2</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct___locale_struct.html' title='tcod_sys::Struct___locale_struct'>Struct___locale_struct</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed3.html' title='tcod_sys::Struct_Unnamed3'>Struct_Unnamed3</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed7.html' title='tcod_sys::Struct_Unnamed7'>Struct_Unnamed7</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed15.html' title='tcod_sys::Struct_Unnamed15'>Struct_Unnamed15</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed17.html' title='tcod_sys::Struct_Unnamed17'>Struct_Unnamed17</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed20.html' title='tcod_sys::Struct_Unnamed20'>Struct_Unnamed20</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed23.html' title='tcod_sys::Struct_Unnamed23'>Struct_Unnamed23</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Union_Unnamed25.html' title='tcod_sys::Union_Unnamed25'>Union_Unnamed25</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed26.html' title='tcod_sys::Struct_Unnamed26'>Struct_Unnamed26</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed27.html' title='tcod_sys::Struct_Unnamed27'>Struct_Unnamed27</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed28.html' title='tcod_sys::Struct_Unnamed28'>Struct_Unnamed28</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct__TCOD_tree_t.html' title='tcod_sys::Struct__TCOD_tree_t'>Struct__TCOD_tree_t</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed29.html' title='tcod_sys::Struct_Unnamed29'>Struct_Unnamed29</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct_Unnamed30.html' title='tcod_sys::Struct_Unnamed30'>Struct_Unnamed30</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod_sys/struct.Struct___va_list_tag.html' title='tcod_sys::Struct___va_list_tag'>Struct___va_list_tag</a>",];implementors['tcod'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/bsp/enum.TraverseOrder.html' title='tcod::bsp::TraverseOrder'>TraverseOrder</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod/colors/struct.Color.html' title='tcod::colors::Color'>Color</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/console/enum.TextAlignment.html' title='tcod::console::TextAlignment'>TextAlignment</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/console/enum.BackgroundFlag.html' title='tcod::console::BackgroundFlag'>BackgroundFlag</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/console/enum.Renderer.html' title='tcod::console::Renderer'>Renderer</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/console/enum.FontLayout.html' title='tcod::console::FontLayout'>FontLayout</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/console/enum.FontType.html' title='tcod::console::FontType'>FontType</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/input/enum.KeyCode.html' title='tcod::input::KeyCode'>KeyCode</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod/input/struct.Key.html' title='tcod::input::Key'>Key</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod/input/struct.Mouse.html' title='tcod::input::Mouse'>Mouse</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod/input/struct.KeyPressFlags.html' title='tcod::input::KeyPressFlags'>KeyPressFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='tcod/input/struct.EventFlags.html' title='tcod::input::EventFlags'>EventFlags</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/input/enum.Event.html' title='tcod::input::Event'>Event</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/map/enum.FovAlgorithm.html' title='tcod::map::FovAlgorithm'>FovAlgorithm</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/noise/enum.NoiseType.html' title='tcod::noise::NoiseType'>NoiseType</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/random/enum.Distribution.html' title='tcod::random::Distribution'>Distribution</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='tcod/random/enum.Algo.html' title='tcod::random::Algo'>Algo</a>",];implementors['rustc_serialize'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/base64/enum.CharacterSet.html' title='rustc_serialize::base64::CharacterSet'>CharacterSet</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/base64/enum.Newline.html' title='rustc_serialize::base64::Newline'>Newline</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='rustc_serialize/base64/struct.Config.html' title='rustc_serialize::base64::Config'>Config</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/base64/enum.FromBase64Error.html' title='rustc_serialize::base64::FromBase64Error'>FromBase64Error</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/hex/enum.FromHexError.html' title='rustc_serialize::hex::FromHexError'>FromHexError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/json/enum.ErrorCode.html' title='rustc_serialize::json::ErrorCode'>ErrorCode</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='rustc_serialize/json/enum.EncoderError.html' title='rustc_serialize::json::EncoderError'>EncoderError</a>",];implementors['input'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/keyboard/struct.ModifierKey.html' title='input::keyboard::ModifierKey'>ModifierKey</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='input/keyboard/enum.Key.html' title='input::keyboard::Key'>Key</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='input/mouse/enum.MouseButton.html' title='input::mouse::MouseButton'>MouseButton</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/controller/struct.ControllerButton.html' title='input::controller::ControllerButton'>ControllerButton</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/controller/struct.ControllerAxisArgs.html' title='input::controller::ControllerAxisArgs'>ControllerAxisArgs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/struct.UpdateArgs.html' title='input::UpdateArgs'>UpdateArgs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/struct.RenderArgs.html' title='input::RenderArgs'>RenderArgs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/struct.AfterRenderArgs.html' title='input::AfterRenderArgs'>AfterRenderArgs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/struct.IdleArgs.html' title='input::IdleArgs'>IdleArgs</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='input/struct.EventId.html' title='input::EventId'>EventId</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='input/enum.Button.html' title='input::Button'>Button</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='enum' href='input/enum.Motion.html' title='input::Motion'>Motion</a>",];implementors['window'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a> for <a class='struct' href='window/struct.Size.html' title='window::Size'>Size</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
