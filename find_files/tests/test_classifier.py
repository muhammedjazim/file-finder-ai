import unittest
from app.models.query_classifier import ClassifyQuery
from app.helpers.classify_query_helper import categorize_user_query_into_required_fields

class ClassifierTest(unittest.TestCase):
    def _non_empty_count(self, instance: ClassifyQuery) -> int:
        """Count attributes that are not default/empty."""
        data = instance.model_dump()
        non_empty = 0
        for key, value in data.items():
            if isinstance(value, bool):
                if value:
                    non_empty += 1
            elif value not in (None, [], ""):
                non_empty += 1
        return non_empty

    def test_classify_query_structure(self):
        """Test that the returned object is a ClassifyQuery with the expected keys and that some fields are filled."""
        query = "Find all PDFs modified in the past year containing AI research"
        result: ClassifyQuery = categorize_user_query_into_required_fields(query)

        self.assertIsInstance(result, ClassifyQuery, "Result should be an instance of ClassifyQuery")

        expected_keys = {
            "date_range",
            "file_types",
            "inside_file_search",
            "project_name",
            "similar_to",
            "person_mentioned",
            "authored_by",
            "recently_accessed",
            "image_metadata"
        }
        result_keys = set(result.model_dump().keys())
        self.assertEqual(result_keys, expected_keys, "Returned keys should match the model definition")

        non_empty_count = self._non_empty_count(result)
        self.assertGreaterEqual(non_empty_count, 1, "At least one field should be filled with meaningful data")

    def test_multiple_queries(self):
        """Test a few sample queries to see variation in filled attributes."""
        sample_queries = [
            "Find all PDFs modified in the past year containing AI research",
            "Find my Python scripts",
            "Show me emails from John",
            "Search for files similar to requirements.txt",
            "Show me all files related to Rust WebAssembly Project",
            "Show me the last 10 files I opened",
            "Find all images containing the text 'invoice'",
            "Search for PDFs authored by Andrew Ng"
        ]

        for query in sample_queries:
            with self.subTest(query=query):
                result: ClassifyQuery = categorize_user_query_into_required_fields(query)
                self.assertIsInstance(result, ClassifyQuery)
                non_empty_count = self._non_empty_count(result)
                self.assertGreaterEqual(
                    non_empty_count,
                    1,
                    f"For query '{query}', expected at least one filled attribute but got {non_empty_count}"
                )

if __name__ == "__main__":
    unittest.main()