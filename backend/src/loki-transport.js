const pino = require('pino');

// Pino transport for shipping logs to Loki
module.exports = (opts) => {
  const lokiUrl = opts.lokiUrl || process.env.LOKI_URL || 'http://loki:3100';

  return pino.transport({
    targets: [
      // Console output (for local development)
      {
        target: 'pino-pretty',
        level: 'info',
        options: {
          colorize: true,
          translateTime: 'SYS:standard',
          ignore: 'pid,hostname'
        }
      },
      // Loki output (for production observability)
      {
        target: 'pino-loki',
        level: 'info',
        options: {
          batching: true,
          interval: 5,
          host: lokiUrl,
          labels: {
            application: 'bbf-backend',
            environment: process.env.NODE_ENV || 'development'
          }
        }
      }
    ]
  });
};
