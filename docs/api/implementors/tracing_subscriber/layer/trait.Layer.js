(function() {var implementors = {};
implementors["tracing_bunyan_formatter"] = [{"text":"impl&lt;S, W&gt; <a class=\"trait\" href=\"tracing_subscriber/layer/trait.Layer.html\" title=\"trait tracing_subscriber::layer::Layer\">Layer</a>&lt;S&gt; for <a class=\"struct\" href=\"tracing_bunyan_formatter/struct.BunyanFormattingLayer.html\" title=\"struct tracing_bunyan_formatter::BunyanFormattingLayer\">BunyanFormattingLayer</a>&lt;W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tracing_core/subscriber/trait.Subscriber.html\" title=\"trait tracing_core::subscriber::Subscriber\">Subscriber</a> + for&lt;'a&gt; <a class=\"trait\" href=\"tracing_subscriber/registry/trait.LookupSpan.html\" title=\"trait tracing_subscriber::registry::LookupSpan\">LookupSpan</a>&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"tracing_subscriber/fmt/writer/trait.MakeWriter.html\" title=\"trait tracing_subscriber::fmt::writer::MakeWriter\">MakeWriter</a> + 'static,&nbsp;</span>","synthetic":false,"types":["tracing_bunyan_formatter::formatting_layer::BunyanFormattingLayer"]},{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"tracing_core/subscriber/trait.Subscriber.html\" title=\"trait tracing_core::subscriber::Subscriber\">Subscriber</a> + for&lt;'a&gt; <a class=\"trait\" href=\"tracing_subscriber/registry/trait.LookupSpan.html\" title=\"trait tracing_subscriber::registry::LookupSpan\">LookupSpan</a>&lt;'a&gt;&gt; <a class=\"trait\" href=\"tracing_subscriber/layer/trait.Layer.html\" title=\"trait tracing_subscriber::layer::Layer\">Layer</a>&lt;S&gt; for <a class=\"struct\" href=\"tracing_bunyan_formatter/struct.JsonStorageLayer.html\" title=\"struct tracing_bunyan_formatter::JsonStorageLayer\">JsonStorageLayer</a>","synthetic":false,"types":["tracing_bunyan_formatter::storage_layer::JsonStorageLayer"]}];
implementors["tracing_subscriber"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()