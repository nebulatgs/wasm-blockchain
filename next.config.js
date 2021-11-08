/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  webpack: (config, options) => {
    config.experiments = config.experiments || {}
    config.experiments.asyncWebAssembly = true
    return config
  },
}
