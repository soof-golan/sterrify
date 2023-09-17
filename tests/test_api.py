

def test_conversion_to_string():
    """Should convert object to string"""
    import sterrify
    obj = [1,2,3]
    assert str(obj) == sterrify.to_string(obj)