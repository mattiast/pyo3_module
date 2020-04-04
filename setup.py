from setuptools import setup
from setuptools_rust import RustExtension, Binding

setup(
    name="sample_module",
    version="0.1.0",
    rust_extensions=[RustExtension("sample_module", binding=Binding.PyO3)],
    setup_requires=["setuptools-rust>=0.10.1", "wheel"],
)
