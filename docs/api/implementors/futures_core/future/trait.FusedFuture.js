(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_channel/oneshot/struct.Receiver.html\" title=\"struct futures_channel::oneshot::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::oneshot::Receiver"]}];
implementors["futures_core"] = [];
implementors["futures_intrusive"] = [{"text":"impl&lt;'a, MutexType, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.ChannelReceiveFuture.html\" title=\"struct futures_intrusive::channel::ChannelReceiveFuture\">ChannelReceiveFuture</a>&lt;'a, MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::channel_future::ChannelReceiveFuture"]},{"text":"impl&lt;'a, MutexType, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.ChannelSendFuture.html\" title=\"struct futures_intrusive::channel::ChannelSendFuture\">ChannelSendFuture</a>&lt;'a, MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::channel_future::ChannelSendFuture"]},{"text":"impl&lt;MutexType, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.ChannelReceiveFuture.html\" title=\"struct futures_intrusive::channel::shared::ChannelReceiveFuture\">ChannelReceiveFuture</a>&lt;MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::channel_future::if_alloc::shared::ChannelReceiveFuture"]},{"text":"impl&lt;MutexType, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.ChannelSendFuture.html\" title=\"struct futures_intrusive::channel::shared::ChannelSendFuture\">ChannelSendFuture</a>&lt;MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::channel_future::if_alloc::shared::ChannelSendFuture"]},{"text":"impl&lt;'a, MutexType, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/struct.StateReceiveFuture.html\" title=\"struct futures_intrusive::channel::StateReceiveFuture\">StateReceiveFuture</a>&lt;'a, MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::state_broadcast::StateReceiveFuture"]},{"text":"impl&lt;MutexType, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/channel/shared/struct.StateReceiveFuture.html\" title=\"struct futures_intrusive::channel::shared::StateReceiveFuture\">StateReceiveFuture</a>&lt;MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::channel::state_broadcast::if_alloc::shared::StateReceiveFuture"]},{"text":"impl&lt;'a, MutexType:&nbsp;<a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/sync/struct.GenericWaitForEventFuture.html\" title=\"struct futures_intrusive::sync::GenericWaitForEventFuture\">GenericWaitForEventFuture</a>&lt;'a, MutexType&gt;","synthetic":false,"types":["futures_intrusive::sync::manual_reset_event::GenericWaitForEventFuture"]},{"text":"impl&lt;'a, MutexType:&nbsp;<a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>, T&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/sync/struct.GenericMutexLockFuture.html\" title=\"struct futures_intrusive::sync::GenericMutexLockFuture\">GenericMutexLockFuture</a>&lt;'a, MutexType, T&gt;","synthetic":false,"types":["futures_intrusive::sync::mutex::GenericMutexLockFuture"]},{"text":"impl&lt;'a, MutexType:&nbsp;<a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/sync/struct.GenericSemaphoreAcquireFuture.html\" title=\"struct futures_intrusive::sync::GenericSemaphoreAcquireFuture\">GenericSemaphoreAcquireFuture</a>&lt;'a, MutexType&gt;","synthetic":false,"types":["futures_intrusive::sync::semaphore::GenericSemaphoreAcquireFuture"]},{"text":"impl&lt;MutexType:&nbsp;<a class=\"trait\" href=\"lock_api/mutex/trait.RawMutex.html\" title=\"trait lock_api::mutex::RawMutex\">RawMutex</a>&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/sync/struct.GenericSharedSemaphoreAcquireFuture.html\" title=\"struct futures_intrusive::sync::GenericSharedSemaphoreAcquireFuture\">GenericSharedSemaphoreAcquireFuture</a>&lt;MutexType&gt;","synthetic":false,"types":["futures_intrusive::sync::semaphore::if_alloc::GenericSharedSemaphoreAcquireFuture"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/timer/struct.LocalTimerFuture.html\" title=\"struct futures_intrusive::timer::LocalTimerFuture\">LocalTimerFuture</a>&lt;'a&gt;","synthetic":false,"types":["futures_intrusive::timer::timer::LocalTimerFuture"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"futures_core/future/trait.FusedFuture.html\" title=\"trait futures_core::future::FusedFuture\">FusedFuture</a> for <a class=\"struct\" href=\"futures_intrusive/timer/struct.TimerFuture.html\" title=\"struct futures_intrusive::timer::TimerFuture\">TimerFuture</a>&lt;'a&gt;","synthetic":false,"types":["futures_intrusive::timer::timer::TimerFuture"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()