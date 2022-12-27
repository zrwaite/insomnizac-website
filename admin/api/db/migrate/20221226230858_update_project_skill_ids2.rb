class UpdateProjectSkillIds2 < ActiveRecord::Migration[7.0]
  def change
    add_column :projects, :skill_ids, :int, array: true #, default: []
  end
end
