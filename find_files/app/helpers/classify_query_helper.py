from app.models.query_classifier import ClassifyQuery
from app.helpers.llm_call_helper import call_dolphin_llama_3

def categorize_user_query_into_required_fields(query: str) -> ClassifyQuery:
    chat_model = call_dolphin_llama_3()
    prompt = f"""
    You are a part of a system that searches for files in a personal computer.
    Analyze this query: "{query}"

    Fill these fields:
    - date_range: Will be filled with any phrase that mentions a date if query contains date-based instructions. Will be a list. eg: past year
    - file_types: Will be filled with a list of file types if query specifies file types
    - inside_file_search: Whether to search inside file contents for matches. Will be a bool.
    - project_name: Will be filled if query references a project. will be a string.
    - similar_to: Find files similar to a given filename or content. Will be a list.
    - person_mentioned: Fill the list with name of a person or multiple persons if their name is present in the query.
    - authored_by: Fill with names of file authors if mentioned in the query. Will be a list.
    - recently_accessed: Fill if query asks for recently accessed files. Will be a bool.
    - image_metadata: Fill if query asks for images. Will be a list

    Do Not return as a JSON

    """

    structured_llm = chat_model.with_structured_output(ClassifyQuery, method='json_mode')
    categorized_query = structured_llm.invoke(prompt)
    return categorized_query