import pytest
from unittest.mock import patch
from ga4gh import Configuration, Transport, ServiceInfo, TES, TesTask


@pytest.fixture
def config():
    return Configuration("http://mockserver")

@pytest.fixture
def transport(config):
    return Transport(config)

@pytest.fixture
def service_info(config):
    return ServiceInfo(config)

@pytest.fixture
def tes(config):
    return TES.new(config)

@patch.object(ServiceInfo, 'get_service_info')
def test_get_service_info(mock_get_service_info, service_info):
    # Mock the return value for get_service_info
    mock_get_service_info.return_value = "Service: {'service': 'GA4GH'}"
    
    result = service_info.get_service_info()
    
    assert result == "Service: {'service': 'GA4GH'}"
    print(result)

@patch.object(Transport, 'get')
def test_transport_get(mock_get, transport):
    # Mock the return value for Transport.get
    mock_get.return_value = '{"key": "value"}'
    
    result = transport.get("test-endpoint", None)
    
    assert result == '{"key": "value"}'
    print(result)

@patch.object(Transport, 'post')
def test_transport_post(mock_post, transport):
    # Mock the return value for Transport.post
    mock_post.return_value = '{"success": true}'
    
    result = transport.post("test-endpoint", '{"data": "test"}')
    
    assert result == '{"success": true}'
    print(result)

@patch.object(Transport, 'put')
def test_transport_put(mock_put, transport):
    # Mock the return value for Transport.put
    mock_put.return_value = '{"updated": true}'
    
    result = transport.put("test-endpoint", '{"data": "update"}')
    
    assert result == '{"updated": true}'
    print(result)

@patch.object(Transport, 'delete')
def test_transport_delete(mock_delete, transport):
    # Mock the return value for Transport.delete
    mock_delete.return_value = '{"deleted": true}'
    
    result = transport.delete("test-endpoint")
    
    assert result == '{"deleted": true}'
    print(result)


def test_configuration():
    # Test creating a Configuration instance
    base_path = "http://example.com/api"
    config = Configuration(base_path)
    
    # Verify the base path was set correctly
    assert config.get_base_path() == base_path
    
    # Test setting a new base path
    new_base_path = "http://example.com/new-api"
    config.set_base_path(new_base_path)
    
    # Verify the new base path
    assert config.get_base_path() == new_base_path

# @patch('ga4gh.TES.create')
# def test_tes_create(mock_tes_create):
#     config = Configuration("http://localhost:8000")
#     tes= TES(config)
#     print(tes)
        
#     # Define the dictionary with the task details
#     task_dict = {
#         "name": "Hello world",
#         "inputs": [{
#             "url": "s3://funnel-bucket/hello.txt",
#             "path": "/inputs/hello.txt"
#         }],
#         "outputs": [{
#             "url": "s3://funnel-bucket/output.txt",
#             "path": "/outputs/stdout"
#         }],
#         "executors": [{
#             "image": "alpine",
#             "command": ["cat", "/inputs/hello.txt"],
#             "stdout": "/outputs/stdout"
#         }]
#     }
        
#     # Create a TesTask instance from the dictionary
#     tes_task = TesTask.from_dict(task_dict)

#     # Now you can use the tes_task object
#     print(tes_task)

if __name__ == "__main__":
    pytest.main()