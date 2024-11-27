# Confido: GA4GH-SDK Extension for Attested-TLS (aTLS) support

## Overview

Confido is a confidential computing middleware that provides support for aTLS protocol as an extension for the GA4GH-SDK/CLI.

## Features

The extension exports the `tls-verifier` method specific to the aTLS protocol. When extension is enabled, this method will be executed by the SDK to perform aTLS Evidence verification for every request to GA4GH API services. The goal of the verification is to ensure that the private part of the certificate was ephemerally generated inside the service's virtual machine (VM) instance working in TEE memory isolation ensured by CPU/GPU hardware encryption. Additionally, it verifies the checksums of the entire remote software stack against a provided trusted repository with reference checksums.

Please reffer to the [demo: Private LLM Chat through Attested-TLS (Confidential Computing)](https://github.com/elixir-cloud-aai/biohackeu24-issues/issues/17) to learn more about the verification flow.

## Installation

Please reffer to the [GA4GH-CLI documentation](https://github.com/elixir-cloud-aai/ga4gh-sdk/blob/main/cli/README.md) for installation instructions.

## Configuration

URL for trusted repository with reference checksums, security mode, and verbose level can be configured for for each service, particularly [TES](https://www.ga4gh.org/product/task-execution-service-tes/):

```json
{
    "TES": {
        "base_path": "https://domain/ga4gh/tes/v1/",
        "extensions": [
            {
                "name": "confido",
                "required": true,
                "configuration": {
                    "trusted-repository": "https://github.com/genxnetwork/confido-trusted-repository",
                    "security-mode": "[enforce|permissive]",
                    "verbose-level": "[off|info|warn|error|debug]"
                }
            }
        ]
    }
}
```

## Usage

Once installed, configured, and enabled, the extension provides a transparent user experience by acting as a security middleware for API connections.

## References

- [Confidential Computing explained by GENXT (5-min video)](https://youtu.be/oiV2IDPX_bk)
- [GA4GH Connect 2023: Enhancing Data Security in GA4GH Task Execution Services with Confidential Computing](https://f1000research.com/posters/13-194)
- [GA4GH Connect 2024 Session "Towards GA4GH-Powered SPEs/TREs"](https://docs.google.com/document/d/1RgFCWumOtk-Ik8UftEqKUR53kuKCm5Y1/)
- [GA4GH Connect 2024: Attested-TLS, Confidential AI demo](https://docs.google.com/presentation/d/11pN19yrnDjoF6G7PMaSKodXGVTsbhfu-keyzrrLFcU0/edit#slide=id.g30076562ae1_0_80)
- [GA4GH Connect 2024: Universal Toolbox for Trust-Enhancing Technologies / Attested-TLS trust model](https://f1000research.com/posters/13-1317)
- [GENXT Confidential LLM Inference (1-min video)](https://youtu.be/oiV2IDPX_bk)