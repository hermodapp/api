docker run --rm -p 4318:4318 -p 4317:4317 \
      -e HONEYCOMB_API_KEY="${HONEYCOMB_API_KEY}" \
      -e HONEYCOMB_DATASET="hermod_local" \
      -v "${PWD}/scripts/collector-config.yaml":/config.yaml \
      --name otelcol otel/opentelemetry-collector \
      --config config.yaml;