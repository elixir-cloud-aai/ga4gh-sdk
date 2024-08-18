import pytest
import ga4gh

def test_service_info():
    # Create a Configuration instance
    base_path = "http://localhost:8000"
    config = ga4gh.Configuration(base_path)
    
    # Create a ServiceInfo instance using the Configuration instance
    service_info = ga4gh.ServiceInfo(config)
    
    # Call the get_service_info method and verify it returns a string
    result = service_info.get_service_info()
    assert isinstance(result, str)
    assert "Service:" in result

if __name__ == "__main__":
    pytest.main()